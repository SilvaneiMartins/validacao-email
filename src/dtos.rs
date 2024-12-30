use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

use crate::models::User;

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize, ToSchema)]
pub struct RegisterUserDto {
    #[validate(length(min = 1, message = "Nome é obrigatório!"))]
    pub name: String,
    #[validate(
        length(min = 1, message = "Email é obrigatório!"),
        email(message = "Email inválido!")
    )]
    pub email: String,
    #[validate(
        length(min = 1, message = "Senha é obrigatório!"),
        length(min = 6, message = "Senha deve ter no minimo 6 caracteres!")
    )]
    pub password: String,
    #[validate(
        length(min = 1, message = "Confirmação de senha é obrigatório!"),
        must_match(other = "password", message = "As senhas não correspondem!")
    )]
    #[serde(rename = "passwordConfirm")]
    pub password_confirmation: String,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize, ToSchema)]
pub struct LoginUserDto {
    #[validate(
        length(min = 1, message = "Email é obrigatório!"),
        email(message = "Email inválido!")
    )]
    pub email: String,
    #[validate(
        length(min = 1, message = "Senha é obrigatório!"),
        length(min = 6, message = "Senha deve ter no minimo 6 caracteres!")
    )]
    pub password: String,
}

#[derive(Serialize, Deserialize, Validate, IntoParams)]
pub struct RequestQueryDto {
    #[validate(range(min = 1))]
    pub page: Option<usize>,
    #[validate(range(min = 1, max = 50))]
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FilterUserDto {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub photo: String,
    pub verified: bool,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

impl FilterUserDto {
    pub fn filter_user(user: &User) -> Self {
        FilterUserDto {
            id: user.id.to_string(),
            name: user.name.to_owned(),
            email: user.email.to_owned(),
            photo: user.photo.to_owned(),
            verified: user.verified,
            role: user.role.to_str().to_string(),
            created_at: user.created_at.unwrap(),
            updated_at: user.updated_at.unwrap(),
        }
    }

    pub fn filter_users(user: &[User]) -> Vec<FilterUserDto> {
        user.iter().map(FilterUserDto::filter_user).collect()
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserData {
    pub user: FilterUserDto,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponseDto {
    pub status: String,
    pub data: UserData,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UsaerListResponseDto {
    pub status: String,
    pub users: Vec<FilterUserDto>,
    pub results: usize,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserLoginResponseDto {
    pub status: String,
    pub token: String,
}

#[derive( Serialize, Deserialize, ToSchema)]
pub struct Response {
    pub status: &'static str,
    pub message: String,
}
