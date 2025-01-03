use actix_web::{
    cookie::time::Duration as ActixWebDuration, cookie::Cookie, web, HttpResponse, Responder, Scope,
};
use serde_json::json;
use validator::Validate;

use crate::{
    auth::RequireAuth,
    db::UserExt,
    dtos::{
        FilterUserDto, LoginUserDto, RegisterUserDto, UserData, UserLoginResponseDto,
        UserResponseDto,
    },
    error::{ErrorMessage, HttpError},
    models::UserRole,
    utils::{password, token},
    AppState,
};

pub fn auth_handler() -> Scope {
    web::scope("/api/auth")
        .route("/register", web::post().to(register))
        .route("/login", web::post().to(login))
        .route(
            "/logout",
            web::post().to(logout).wrap(RequireAuth::allowed_roles(vec![
                UserRole::User,
                UserRole::Moderator,
                UserRole::Admin,
            ])),
        )
}

#[utoipa::path(
    post,
    path = "/api/auth/register",
    tag = "Endpoint de registro de conta de usuário",
    request_body(
        content = RegisterUserDto,
        description = "Dados do usuário a ser registrado",
        example = json!({
            "email": "johndoe@email.com",
            "name": "John Doe",
            "password": "123456",
            "passwordConfirm": "123456"
        }),
    ),
    responses(
        (status=201, description="Usuário registrado com sucesso!", body=UserResponseDto),
        (status=400, description="Erro de validação!", body=Response),
        (status=409, description="Email já cadastrado!", body=Response),
        (status=500, description="Erro interno do servidor!", body=Response),
    )
)]
pub async fn register(
    app_state: web::Data<AppState>,
    body: web::Json<RegisterUserDto>,
) -> Result<HttpResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bat_request(e.to_string()))?;

    let hashed_password =
        password::hash(&body.password).map_err(|e| HttpError::server_error(e.to_string()))?;

    let result = app_state
        .db_client
        .save_user(&body.name, &body.email, &hashed_password)
        .await;

    match result {
        Ok(user) => Ok(HttpResponse::Created().json(UserResponseDto {
            status: "success".to_string(),
            data: UserData {
                user: FilterUserDto::filter_user(&user),
            },
        })),
        Err(sqlx::Error::Database(db_err)) => {
            if db_err.is_unique_violation() {
                Err(HttpError::unique_constraint_violation(
                    ErrorMessage::EmailExist,
                ))
            } else {
                Err(HttpError::server_error(db_err.to_string()))
            }
        }
        Err(e) => Err(HttpError::server_error(e.to_string())),
    }
}

#[utoipa::path(
    post,
    path = "/api/auth/login",
    tag = "Endpoint de login de usuário",
    request_body(
        content = LoginUserDto,
        description = "Credenciais para efetuar login na sua conta",
        example = json!({
        "email": "johndoe@email.com",
        "password": "123456",
    })),
    responses(
        (status=200, description="Usuário logado com sucesso!", body=UserLoginResponseDto),
        (status=400, description="Erro de validação!", body=Response),
        (status=401, description="Credenciais inválidas!", body=Response),
        (status=500, description="Erro interno do servidor!", body=Response),
    )
)]
pub async fn login(
    app_state: web::Data<AppState>,
    body: web::Json<LoginUserDto>,
) -> Result<HttpResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let result = app_state
        .db_client
        .get_user(None, None, Some(&body.email))
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let user = result.ok_or(HttpError::unauthorized(ErrorMessage::WrongCredentials))?;

    let password_matches = password::compare(&body.password, &body.email)
        .map_err(|_| HttpError::unauthorized(ErrorMessage::WrongCredentials))?;

    if password_matches {
        let token = token::create_token(
            &user.id.to_string(),
            &app_state.env.jwt_secret.as_bytes(),
            app_state.env.jwt_maxage,
        )
        .map_err(|e| HttpError::server_error(e.to_string()))?;

        let cookie = Cookie::build("token", token.to_owned())
            .path("/")
            .max_age(ActixWebDuration::new(60 * &app_state.env.jwt_maxage, 0))
            .http_only(true)
            .finish();
        Ok(HttpResponse::Ok()
            .cookie(cookie)
            .json(UserLoginResponseDto {
                status: "success".to_string(),
                token,
            }))
    } else {
        Err(HttpError::unauthorized(ErrorMessage::WrongCredentials))
    }
}

#[utoipa::path(
    post,
    path = "/api/auth/logout",
    tag = "Endpoint de logout de usuário",
    responses(
        (status=200, description="Usuário deslogado com sucesso!", body=Response),
        (status=400, description="Erro de validação!", body=Response),
        (status=401, description="Não autorizado!", body=Response),
        (status=500, description="Erro interno do servidor!", body=Response),
    ),
    security(
        ("token" = [])
    )
)]
pub async fn logout() -> impl Responder {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success"}))
}
