use std::sync::Arc;

use crate::{
    application::security::{
        jwt::JwtService,
        password_hasher::PasswordHasher,
    },
    domain::user::repository::UserRepository,
    shared::config::AppConfig,
};
use crate::domain::auth::repository::RefreshTokenRepository;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,

    pub user_repo: Arc<dyn UserRepository>,
    pub refresh_token_repo: Arc<dyn RefreshTokenRepository>,

    pub password_hasher: Arc<dyn PasswordHasher>,
    pub jwt_service: Arc<dyn JwtService>,
}
