use uuid::Uuid;

use crate::domain::user::value_objects::{UserEmail, UserName};

#[derive(Debug, Clone)]
pub struct User {
    id: Uuid,
    name: UserName,
    email: UserEmail,
    password_hash: String,
}

impl User {
    pub fn register(
        id: Uuid,
        name: UserName,
        email: UserEmail,
        password_hash: String,
    ) -> Self {
        Self {
            id,
            name,
            email,
            password_hash,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn email(&self) -> &UserEmail {
        &self.email
    }

    pub fn name(&self) -> &UserName {
        &self.name
    }

    pub fn password_hash(&self) -> &str {
        &self.password_hash
    }
}
