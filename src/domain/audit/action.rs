#[derive(Debug, Clone, Copy)]
pub enum AuditAction {
    LoginSuccess,
    LoginFailed,
}

impl AuditAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            AuditAction::LoginSuccess => "LOGIN_SUCCESS",
            AuditAction::LoginFailed => "LOGIN_FAILED",
        }
    }
}
