#[derive(Debug, Clone, Copy)]
pub enum AuditAction {
    LoginSuccess,
    LoginFailed,
    ChangePasswordSuccess,
    ChangePasswordFailed,
}

impl AuditAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            AuditAction::LoginSuccess => "LOGIN_SUCCESS",
            AuditAction::LoginFailed => "LOGIN_FAILED",
            AuditAction::ChangePasswordSuccess => "CHANGE_PASSWORD_SUCCESS",
            AuditAction::ChangePasswordFailed => "CHANGE_PASSWORD_FAILED",
        }
    }
}
