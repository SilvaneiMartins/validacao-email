use actix_web::{web, HttpResponse, Scope};
use validator::Validate;

use crate::{
    auth::{Authenticated, RequireAuth},
    db::UserExt,
    dtos::{FilterUserDto, RequestQueryDto, UserData, UserListResponseDto, UserResponseDto},
    error::HttpError,
    AppState,
    models::UserRole,
};

pub fn users_handler() -> Scope {
    web::scope("/api/users")
        .route(
            "",
            web::get()
                .to(get_users)
                .wrap(RequireAuth::allowed_roles(vec![UserRole::Admin])),
        )
        .route(
            "/me",
            web::get()
                .to(get_me)
                .wrap(RequireAuth::allowed_roles(vec![
                    UserRole::User,
                    UserRole::Moderator,
                    UserRole::Admin,
                ])),
        )
}

#[utoipa::path(
    get,
    path = "/api/users/me",
    tag = "Endpoint de informações do usuário autenticado",
    responses(
        (status=200, description="Informações do usuário autenticado!", body=UserResponseDto),
        (status=500, description="Erro interno do servidor!", body=Response),
    )
)]
pub async fn get_me(user: Authenticated) -> Result<HttpResponse, HttpError> {
    let filterd_user =  FilterUserDto::filter_user(&user);

    let response_date = UserResponseDto {
        status: "success".to_string(),
        data: UserData {
            user: filterd_user,
        }
    };

    Ok(HttpResponse::Ok().json(response_date))
}

#[utoipa::path(
    get,
    path = "/api/users",
    tag = "Endpoint de listagem de usuários",
    params(
        RequestQueryDto
    ),
    responses(
        (status=200, description="Lista de todos os usuários", body=[UserListResponseDto]),
        (status=401, description="Não autorizado!", body=Response),
        (status=403, description="Acesso negado!", body=Response),
        (status=500, description="Erro interno do servidor!", body=Response),
    )
)]
pub async fn get_users(
    query: web::Query<RequestQueryDto>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let query_params: RequestQueryDto = query.into_inner();

    query_params
        .validate()
        .map_err(|e| HttpError::bat_request(e.to_string()))?;

    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(10);

    let users = app_state
        .db_client
        .get_users(page as u32, limit)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(HttpResponse::Ok().json(UserListResponseDto {
        status: "success".to_string(),
        users: FilterUserDto::filter_users(&users),
        results: users.len(),
    }))
}
