use application::UserRepository;
use domain::{CreateUserRequest, UpdateUserRequest, User, UserError};
use r2d2::Pool;
use r2d2_mysql::mysql::{prelude::*, params, Opts, OptsBuilder};
use r2d2_mysql::MySqlConnectionManager;
use std::sync::Arc;
use uuid::Uuid;

pub struct MySqlUserRepository {
    pool: Arc<Pool<MySqlConnectionManager>>,
}

impl MySqlUserRepository {
    pub fn new(pool: Arc<Pool<MySqlConnectionManager>>) -> Self {
        Self { pool }
    }

    pub fn init_db(&self) -> Result<(), UserError> {
        let mut conn = self.pool.get().map_err(|e| UserError::DatabaseError(e.to_string()))?;
        conn.query_drop(
            r#"CREATE TABLE IF NOT EXISTS users (
                id CHAR(36) PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                email VARCHAR(255) NOT NULL
            )"#
        ).map_err(|e| UserError::DatabaseError(e.to_string()))?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl UserRepository for MySqlUserRepository {
    async fn find_all(&self) -> Result<Vec<User>, UserError> {
        let mut conn = self.pool.get().map_err(|e| UserError::DatabaseError(e.to_string()))?;
        let users = conn.query_map(
            "SELECT id, name, email FROM users",
            |(id, name, email): (String, String, String)| User {
                id: Uuid::parse_str(&id).unwrap_or(Uuid::nil()),
                name,
                email,
            },
        ).map_err(|e| UserError::DatabaseError(e.to_string()))?;
        Ok(users)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<User, UserError> {
        let mut conn = self.pool.get().map_err(|e| UserError::DatabaseError(e.to_string()))?;
        let result: Option<(String, String, String)> = conn.exec_first(
            "SELECT id, name, email FROM users WHERE id = :id",
            params! { "id" => id.to_string() },
        ).map_err(|e| UserError::DatabaseError(e.to_string()))?;
        match result {
            Some((id, name, email)) => Ok(User { id: Uuid::parse_str(&id).unwrap_or(Uuid::nil()), name, email }),
            None => Err(UserError::NotFound(id)),
        }
    }

    async fn create(&self, user: CreateUserRequest) -> Result<User, UserError> {
        let mut conn = self.pool.get().map_err(|e| UserError::DatabaseError(e.to_string()))?;
        let id = Uuid::new_v4();
        conn.exec_drop(
            "INSERT INTO users (id, name, email) VALUES (:id, :name, :email)",
            params! { "id" => id.to_string(), "name" => &user.name, "email" => &user.email },
        ).map_err(|e| UserError::DatabaseError(e.to_string()))?;
        Ok(User { id, name: user.name, email: user.email })
    }

    async fn update(&self, id: Uuid, user: UpdateUserRequest) -> Result<User, UserError> {
        let mut conn = self.pool.get().map_err(|e| UserError::DatabaseError(e.to_string()))?;
        let current = self.find_by_id(id).await?;
        let name = user.name.unwrap_or(current.name);
        let email = user.email.unwrap_or(current.email);
        conn.exec_drop(
            "UPDATE users SET name = :name, email = :email WHERE id = :id",
            params! { "id" => id.to_string(), "name" => &name, "email" => &email },
        ).map_err(|e| UserError::DatabaseError(e.to_string()))?;
        Ok(User { id, name, email })
    }

    async fn delete(&self, id: Uuid) -> Result<(), UserError> {
        let mut conn = self.pool.get().map_err(|e| UserError::DatabaseError(e.to_string()))?;
        conn.exec_drop(
            "DELETE FROM users WHERE id = :id",
            params! { "id" => id.to_string() },
        ).map_err(|e| UserError::DatabaseError(e.to_string()))?;
        Ok(())
    }
}
