use std::fmt;

use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};

use crate::dtos::Response;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorMessage {
    EmptyPassword,
    ExceededMaxPasswordLength(usize),
    HashingError,
    InvalidHashFormate,
    InvalidToken,
    ServerError,
    WrongCredentials,
    EmailExist,
    UserNoLongerExist,
    TokenNotProvided,
    PermissionDenied,
}

impl ToString for ErrorMessage {
    fn to_string(&self) -> String {
        self.to_str().to_owned()
    }
}

impl Into<String> for ErrorMessage {
    fn into(self) -> String {
        self.to_string()
    }
}

impl ErrorMessage {
    fn to_str(&self) -> String {
        match self {
            ErrorMessage::ServerError => "Erro no servidor. Tente novamente mais tarde.".to_string(),
            ErrorMessage::WrongCredentials => "E-mail ou senha estão incorretos.".to_string(),
            ErrorMessage::EmailExist => "Já existe um usuário com este e-mail.".to_string(),
            ErrorMessage::UserNoLongerExist => "O usuário pertencente a este token não existe mais.".to_string(),
            ErrorMessage::EmptyPassword => "A senha não pode estar vazia.".to_string(),
            ErrorMessage::HashingError => "Erro ao fazer hash da senha.".to_string(),
            ErrorMessage::InvalidHashFormate => "Formato de hash de senha inválido.".to_string(),
            ErrorMessage::ExceededMaxPasswordLength(max_length) => format!("A senha não deve ter mais de {} caracteres.", max_length),
            ErrorMessage::InvalidToken => "O token de autenticação é inválido ou expirou.".to_string(),
            ErrorMessage::TokenNotProvided => "Você não está logado, por favor forneça um token.".to_string(),
            ErrorMessage::PermissionDenied => "Você não tem permissão para executar esta ação.".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpError {
    pub message: String,
    pub status: u16,
}

impl HttpError {
    pub fn new(message: impl Into<String>, status: u16) -> Self {
        HttpError {
            message: message.into(),
            status
        }
    }

    pub fn server_error(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: 500
        }
    }

    pub fn bat_request(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: 400
        }
    }

    pub fn unique_constraint_violation(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: 409
        }
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: 401
        }
    }

    pub fn into_http_response(self) -> HttpResponse {
        match self.status {
            400 => HttpResponse::BadRequest().json(Response {
                status: "fail",
                message: self.message.into()
            }),
            401 => HttpResponse::Unauthorized().json(Response {
                status: "fail",
                message: self.message.into()
            }),
            409 => HttpResponse::Conflict().json(Response {
                status: "fail",
                message: self.message.into()
            }),
            500 => HttpResponse::InternalServerError().json(Response {
                status: "fail",
                message: self.message.into()
            }),
            _ => {
                eprintln!(
                    "Aguarde: Falta correspondência de padrão. Código de status convertido {} para 500.",
                    self.status
                );

                HttpResponse::InternalServerError().json(Response {
                    status: "error",
                    message: ErrorMessage::ServerError.into()
                })
            }
        }
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HttpError {{ message: {}, status: {} }}",
            self.message, self.status
        )
    }
}

impl std::error::Error for HttpError {}

impl ResponseError for HttpError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
       let cloned = self.clone();
       cloned.into_http_response()
    }
}
