pub mod health {
    pub const HEALTH_OK: &str = "Service is running";
}

pub mod auth {
    pub const UNAUTHORIZED_MISSING_HEADER: &str = "missing authorization header";
    pub const UNAUTHORIZED_INVALID_HEADER: &str = "invalid authorization header";
    pub const UNAUTHORIZED_INVALID_TOKEN: &str = "invalid or expired token";
    pub const INVALID_CREDENTIALS: &str = "invalid email or password";
    pub const LOGIN_FAILED: &str = "failed to login";
    pub const LOGIN_SUCCESS: &str = "login successful";
    pub const TOKEN_GENERATION_FAILED: &str = "failed to generate access token";
    pub const INVALID_REFRESH_TOKEN: &str = "invalid refresh token";
    pub const LOGOUT_FAILED: &str = "logout failed";
    pub const LOGOUT_SUCCESS: &str = "logout successful";
    pub const REFRESH_TOKEN_FAILED: &str = "failed to refresh token";
    pub const REFRESH_TOKEN_SUCCESS: &str = "token refreshed";
    pub const HASHING_FAILED: &str = "failed to process password";
    pub const EMAIL_ALREADY_EXISTS: &str = "email already registered";
    pub const REGISTER_USER_FAILED: &str = "failed to register user";
    pub const REGISTER_USER_SUCCESS: &str = "user registered successfully";
    pub const RATE_LIMIT_EXCEEDED: &str = "too many requests";
}

pub mod users {
    pub const WEAK_PASSWORD: &str = "password does not meet requirements";
    pub const CHANGE_PASSWORD_FAILED: &str = "failed to change password";
    pub const CHANGE_PASSWORD_SUCCESS: &str = "password updated successfully";
    pub const USER_NOT_FOUND: &str = "user not found";
    pub const GET_CURRENT_USER_FAILED: &str = "failed to fetch current user";
    pub const GET_CURRENT_USER_SUCCESS: &str = "current user fetched";
    pub const UPDATE_PROFILE_FAILED: &str = "failed to update profile";
    pub const UPDATE_PROFILE_SUCCESS: &str = "profile updated successfully";
}

pub mod validator {
    pub const INVALID_CURRENT_PASSWORD: &str = "invalid current password";
    pub const INVALID_USER_DATA: &str = "invalid user data";
    pub const INVALID_PROFILE_DATA: &str = "invalid profile data";
    pub const INVALID_PASSWORD_FORMAT: &str = "password must be at least 8 characters";
}
