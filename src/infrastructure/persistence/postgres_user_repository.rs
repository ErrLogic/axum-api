use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::domain::user::{
    entity::User,
    repository::{UserRepository, UserRepositoryError},
    value_objects::{UserEmail, UserName},
};

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<User, UserRepositoryError> {
        let row = sqlx::query(
            r#"
            SELECT id, name, email, password_hash
            FROM users
            WHERE id = $1
            "#,
        )
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|_| UserRepositoryError::Unknown)?;

        let row = match row {
            Some(r) => r,
            None => return Err(UserRepositoryError::NotFound),
        };

        let name = UserName::new(row.get::<String, _>("name"))
            .map_err(|_| UserRepositoryError::Domain)?;

        let email = UserEmail::new(row.get::<String, _>("email"))
            .map_err(|_| UserRepositoryError::Domain)?;

        Ok(User::register(
            row.get("id"),
            name,
            email,
            row.get("password_hash"),
        ))
    }

    async fn find_by_email(
        &self,
        email: &str,
    ) -> Result<User, UserRepositoryError> {
        let row = sqlx::query(
            r#"
            SELECT id, name, email, password_hash
            FROM users
            WHERE email = $1
            "#,
        )
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            .map_err(|_| UserRepositoryError::Unknown)?;

        let row = match row {
            Some(r) => r,
            None => return Err(UserRepositoryError::NotFound),
        };

        let name = UserName::new(row.get::<String, _>("name"))
            .map_err(|_| UserRepositoryError::Domain)?;

        let email = UserEmail::new(row.get::<String, _>("email"))
            .map_err(|_| UserRepositoryError::Domain)?;

        Ok(User::register(
            row.get("id"),
            name,
            email,
            row.get("password_hash"),
        ))
    }

    async fn save(&self, user: &User) -> Result<(), UserRepositoryError> {
        let result = sqlx::query(
            r#"
            INSERT INTO users (id, name, email, password_hash)
            VALUES ($1, $2, $3, $4)
            "#,
        )
            .bind(user.id())
            .bind(user.name().value())
            .bind(user.email().value())
            .bind(user.password_hash())
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                // unique violation (email)
                if let Some(db_err) = e.as_database_error() {
                    if db_err.code() == Some(std::borrow::Cow::Borrowed("23505")) {
                        return Err(UserRepositoryError::Conflict);
                    }
                }

                Err(UserRepositoryError::Unknown)
            }
        }
    }
}
