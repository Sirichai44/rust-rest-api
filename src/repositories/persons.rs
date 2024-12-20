use std::sync::Arc;

use axum::async_trait;
use mockall::automock;

use crate::entities::persons::Person;
use mongodb::{bson::oid::ObjectId, error::Error};

pub type SharedPersonsRepository = Arc<dyn PersonsRepository + Send + Sync>;

#[async_trait]
#[automock]
pub trait PersonsRepository {
    async fn create(&self, person: Person) -> Result<ObjectId, Error>;
    async fn find_by_email(&self, email: String) -> Result<Person, Error>;
}
