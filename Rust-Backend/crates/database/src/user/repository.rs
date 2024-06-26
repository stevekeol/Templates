use std::sync::Arc;

use async_trait::async_trait;
use mongodb::{
    bson::{doc, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
};
use tokio_stream::StreamExt;

use crate::{user::model::User, Database};
use utils::AppResult;

pub type DynUserRepository = Arc<dyn UserRepositoryTrait + Send + Sync>;

// 主要用于Service中，表示提供了该Trait功能
#[async_trait]
pub trait UserRepositoryTrait {
    async fn create_user(
        &self,
        name: &str,
        email: &str,
        password: &str,
    ) -> AppResult<InsertOneResult>;

    async fn get_user_by_id(&self, id: &str) -> AppResult<Option<User>>;

    async fn get_user_by_email(&self, email: &str) -> AppResult<Option<User>>;

    async fn update_user(
        &self,
        id: &str,
        name: &str,
        email: &str,
        password: &str,
    ) -> AppResult<UpdateResult>;

    async fn delete_user(&self, id: &str) -> AppResult<DeleteResult>;

    async fn get_all_users(&self) -> AppResult<Vec<User>>;
}

#[async_trait]
impl UserRepositoryTrait for Database {
    async fn create_user(
        &self,
        name: &str,
        email: &str,
        password: &str,
    ) -> AppResult<InsertOneResult> {
        let new_doc = User {
            id: None,
            name: name.to_string(),
            email: email.to_string(),
            password: password.to_string(),
        };

        let user = self.users.insert_one(new_doc, None).await?;

        Ok(user)
    }

    async fn get_user_by_email(&self, email: &str) -> AppResult<Option<User>> {
        let filter = doc! {"email": email};
        let user_detail = self.users.find_one(filter, None).await?;

        Ok(user_detail)
    }

    async fn get_user_by_id(&self, id: &str) -> AppResult<Option<User>> {
        let obj_id = ObjectId::parse_str(id)?;
        let filter = doc! {"_id": obj_id};
        let user_detail = self.users.find_one(filter, None).await?;

        Ok(user_detail)
    }

    async fn update_user(
        &self,
        id: &str,
        name: &str,
        email: &str,
        password: &str,
    ) -> AppResult<UpdateResult> {
        let id = ObjectId::parse_str(id)?;
        let filter = doc! {"_id": id};
        let new_doc = doc! {
            "$set":
                {
                    "name": name,
                    "email": email,
                    "password": password,
                },
        };

        let updated_doc = self.users.update_one(filter, new_doc, None).await?;

        Ok(updated_doc)
    }

    async fn delete_user(&self, id: &str) -> AppResult<DeleteResult> {
        let obj_id = ObjectId::parse_str(id)?;
        let filter = doc! {"_id": obj_id};
        let user_detail = self.users.delete_one(filter, None).await?;

        Ok(user_detail)
    }

    async fn get_all_users(&self) -> AppResult<Vec<User>> {
        let mut cursor = self.users.find(None, None).await?;

        let mut users: Vec<User> = Vec::new();
        while let Some(doc) = cursor.next().await {
            users.push(doc?);
        }

        Ok(users)
    }
}
