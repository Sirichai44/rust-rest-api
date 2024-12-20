use axum::async_trait;
use bson::Document;
use chrono::{DateTime, Utc};
use std::sync::Arc;

use crate::entities::persons::Person;
use mongodb::{
    bson::{doc, oid::ObjectId, DateTime as BSONDateTime},
    error::Error,
    Collection,
};

use super::persons::{PersonsRepository, SharedPersonsRepository};

pub struct UserRepository {
    db_mongo: mongodb::Database,
}

impl UserRepository {
    pub fn new(db_mongo: mongodb::Database) -> SharedPersonsRepository {
        Arc::new(Self { db_mongo })
    }
}

#[async_trait]
impl PersonsRepository for UserRepository {
    async fn create(&self, person: Person) -> Result<ObjectId, Error> {
        let collection = self.db_mongo.collection("users");
        let created_date: DateTime<Utc> =
            DateTime::from_naive_utc_and_offset(person.created_at, Utc);
        let updated_date: DateTime<Utc> =
            DateTime::from_naive_utc_and_offset(person.updated_at, Utc);

        let doc = doc! {
            "_id": person.id,
            "name": person.name,
            "email": person.email,
            "created_at": BSONDateTime::from_millis(created_date.timestamp_millis()),
            "updated_at": BSONDateTime::from_millis(updated_date.timestamp_millis()),
        };

        let result = collection.insert_one(doc, None).await?;

        Ok(result.inserted_id.as_object_id().unwrap().to_owned())
    }

    async fn find_by_email(&self, email: String) -> Result<Person, Error> {
        let collection: Collection<Document> = self.db_mongo.collection("users");

        let filter = doc! {
            "email": email
        };

        let result = collection.find_one(filter, None).await?;

        match result {
            Some(doc) => {
                let created_at = doc
                    .get_datetime("created_at")
                    .unwrap()
                    .to_owned()
                    .try_to_rfc3339_string()
                    .unwrap();
                let updated_at = doc
                    .get_datetime("updated_at")
                    .unwrap()
                    .to_owned()
                    .try_to_rfc3339_string()
                    .unwrap();

                let person = Person {
                    id: doc.get_object_id("_id").unwrap().to_owned(),
                    name: doc.get_str("name").unwrap().to_owned(),
                    email: doc.get_str("email").unwrap().to_owned(),
                    created_at: DateTime::parse_from_rfc3339(&created_at)
                        .unwrap()
                        .naive_utc(),
                    updated_at: DateTime::parse_from_rfc3339(&updated_at)
                        .unwrap()
                        .naive_utc(),
                };

                Ok(person)
            }
            None => Err(Error::from(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "User not found",
            ))),
        }
    }
}
