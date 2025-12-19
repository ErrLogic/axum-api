use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::domain::auth::{
    refresh_token::RefreshToken,
    repository::{RefreshTokenRepository, RefreshTokenRepositoryError},
};

pub struct PostgresRefreshTokenRepository {
    pool: PgPool,
}

impl PostgresRefreshTokenRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl RefreshTokenRepository for PostgresRefreshTokenRepository {
    async fn store(&self, token: RefreshToken) -> Result<(), RefreshTokenRepositoryError> {
        sqlx::query(
            r#"
            INSERT INTO refresh_tokens (id, user_id, token, expires_at, revoked_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(token.id)
        .bind(token.user_id)
        .bind(token.token)
        .bind(token.expires_at)
        .bind(token.revoked_at)
        .execute(&self.pool)
        .await
        .map_err(|_| RefreshTokenRepositoryError::Unexpected)?;

        Ok(())
    }

    async fn find_by_token(
        &self,
        token: &str,
    ) -> Result<RefreshToken, RefreshTokenRepositoryError> {
        let row = sqlx::query(
            r#"
            SELECT id, user_id, token, expires_at, revoked_at
            FROM refresh_tokens
            WHERE token = $1
            "#,
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| RefreshTokenRepositoryError::Unexpected)?;

        let row = match row {
            Some(r) => r,
            None => return Err(RefreshTokenRepositoryError::NotFound),
        };

        Ok(RefreshToken {
            id: row.get::<Uuid, _>("id"),
            user_id: row.get::<Uuid, _>("user_id"),
            token: row.get::<String, _>("token"),
            expires_at: row.get::<DateTime<Utc>, _>("expires_at"),
            revoked_at: row.get::<Option<DateTime<Utc>>, _>("revoked_at"),
        })
    }

    async fn revoke(&self, id: Uuid) -> Result<(), RefreshTokenRepositoryError> {
        let result = sqlx::query(
            r#"
            UPDATE refresh_tokens
            SET revoked_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|_| RefreshTokenRepositoryError::Unexpected)?;

        if result.rows_affected() == 0 {
            return Err(RefreshTokenRepositoryError::NotFound);
        }

        Ok(())
    }

    async fn revoke_by_user(&self, user_id: Uuid) -> Result<(), RefreshTokenRepositoryError> {
        sqlx::query(
            r#"
            UPDATE refresh_tokens
            SET revoked_at = NOW()
            WHERE user_id = $1
                AND revoked_at IS NULL
            "#,
        )
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(|_| RefreshTokenRepositoryError::Unexpected)?;

        Ok(())
    }
}
