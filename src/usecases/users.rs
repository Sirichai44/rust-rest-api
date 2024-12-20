use std::sync::Arc;

use bson::oid::ObjectId;

use crate::{
    entities::persons::Person as PersonsEntity,
    models::{
        error::{APIError, IntoErrorResponse},
        // persons::Person,
    },
    repositories::persons::SharedPersonsRepository,
    time_helper::IntoTimerHelperShared,
};

pub struct UsersUseCase {
    persons_repository: SharedPersonsRepository,
    time_helper: IntoTimerHelperShared,
}

impl UsersUseCase {
    pub fn creation(
        persons_repository: SharedPersonsRepository,
        time_helper: IntoTimerHelperShared,
    ) -> Arc<Self> {
        Arc::new(Self {
            persons_repository,
            time_helper,
        })
    }

    pub async fn adding(
        &self,
        name: String,
        email: String,
    ) -> Result<ObjectId, Box<dyn IntoErrorResponse>> {
        if let Ok(_) = self.persons_repository.find_by_email(email.clone()).await {
            return Err(Box::new(APIError::PersonAlreadyExistsError(
                email.clone(),
                "already exists".to_string(),
            )));
        }

        let person = PersonsEntity::new(name, email, self.time_helper.clone());
        let inserted_id = match self.persons_repository.create(person).await {
            Ok(inserted_id) => inserted_id,
            Err(e) => return Err(Box::new(APIError::AddPersonError(e))),
        };

        Ok(inserted_id)
    }
}
