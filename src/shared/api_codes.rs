pub mod health {
    pub const HEALTH_OK: &str = "HEALTH_OK";
}

pub mod auth {
    pub const UNAUTHORIZED: &str = "UNAUTHORIZED";
    pub const INVALID_CREDENTIALS: &str = "INVALID_CREDENTIALS";
    pub const LOGIN_FAILED: &str = "LOGIN_FAILED";
    pub const LOGIN_SUCCESS: &str = "LOGIN_SUCCESS";
    pub const TOKEN_GENERATION_FAILED: &str = "TOKEN_GENERATION_FAILED";
    pub const INVALID_REFRESH_TOKEN: &str = "INVALID_REFRESH_TOKEN";
    pub const LOGOUT_FAILED: &str = "LOGOUT_FAILED";
    pub const LOGOUT_SUCCESS: &str = "LOGOUT_SUCCESS";
    pub const REFRESH_TOKEN_FAILED: &str = "REFRESH_TOKEN_FAILED";
    pub const REFRESH_TOKEN_SUCCESS: &str = "REFRESH_TOKEN_SUCCESS";
    pub const HASHING_FAILED: &str = "HASHING_FAILED";
    pub const EMAIL_ALREADY_EXISTS: &str = "EMAIL_ALREADY_EXISTS";
    pub const REGISTER_USER_FAILED: &str = "REGISTER_USER_FAILED";
    pub const REGISTER_USER_SUCCESS: &str = "REGISTER_USER_SUCCESS";
    pub const RATE_LIMIT_EXCEEDED: &str = "RATE_LIMIT_EXCEEDED";
}

pub mod users {
    pub const WEAK_PASSWORD: &str = "WEAK_PASSWORD";
    pub const CHANGE_PASSWORD_FAILED: &str = "CHANGE_PASSWORD_FAILED";
    pub const CHANGE_PASSWORD_SUCCESS: &str = "CHANGE_PASSWORD_SUCCESS";
    pub const USER_NOT_FOUND: &str = "USER_NOT_FOUND";
    pub const GET_CURRENT_USER_FAILED: &str = "GET_CURRENT_USER_FAILED";
    pub const GET_CURRENT_USER_SUCCESS: &str = "GET_CURRENT_USER_SUCCESS";
    pub const UPDATE_PROFILE_FAILED: &str = "UPDATE_PROFILE_FAILED";
    pub const UPDATE_PROFILE_SUCCESS: &str = "UPDATE_PROFILE_SUCCESS";
}

pub mod validator {
    pub const VALIDATION_ERROR: &str = "VALIDATION_ERROR";
}
