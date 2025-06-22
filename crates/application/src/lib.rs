use domain::{CreateUserRequest, UpdateUserRequest, User, UserError};
use std::sync::Arc;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<User>, UserError>;
    async fn find_by_id(&self, id: Uuid) -> Result<User, UserError>;
    async fn create(&self, user: CreateUserRequest) -> Result<User, UserError>;
    async fn update(&self, id: Uuid, user: UpdateUserRequest) -> Result<User, UserError>;
    async fn delete(&self, id: Uuid) -> Result<(), UserError>;
}

pub struct UserService<R: UserRepository> {
    repository: Arc<R>,
}

impl<R: UserRepository> UserService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    pub async fn get_users(&self) -> Result<Vec<User>, UserError> {
        self.repository.find_all().await
    }

    pub async fn get_user(&self, id: Uuid) -> Result<User, UserError> {
        self.repository.find_by_id(id).await
    }

    pub async fn create_user(&self, req: CreateUserRequest) -> Result<User, UserError> {
        // ここに必要なバリデーションを追加できます
        if req.name.is_empty() {
            return Err(UserError::ValidationError("名前は空にできません".to_string()));
        }
        if req.email.is_empty() {
            return Err(UserError::ValidationError("メールは空にできません".to_string()));
        }

        self.repository.create(req).await
    }

    pub async fn update_user(&self, id: Uuid, req: UpdateUserRequest) -> Result<User, UserError> {
        self.repository.update(id, req).await
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<(), UserError> {
        self.repository.delete(id).await
    }
}
