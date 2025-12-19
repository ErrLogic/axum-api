use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct AuthContext {
    pub user_id: Uuid,
}
