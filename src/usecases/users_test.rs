#[cfg(test)]
mod tests {
    use bson::oid::ObjectId;
    use mockall::predicate::eq;
    use std::sync::Arc;

    use crate::{
        models::persons::Person, repositories::persons::MockPersonsRepository,
        time_helper::TimerHelper, usecases::users::UsersUseCase,
    };

    #[tokio::test]
    async fn adding_test() {
        let mut persons_repository_mock = MockPersonsRepository::new();
        let time_helper = TimerHelper::Mock.creation();

        let req = Person {
            name: "test".to_string(),
            email: "test@mail.com".to_string(),
        };

        persons_repository_mock
            .expect_find_by_email()
            .with(eq(req.email.clone()))
            .times(1)
            .returning(|_| {
                Box::pin(async {
                    Err(mongodb::error::Error::from(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "not found",
                    )))
                })
            });

        persons_repository_mock
            .expect_create()
            .withf(|person| person.name == "test" && person.email == "test@mail.com")
            .times(1)
            .returning(|_| Box::pin(async { Ok(ObjectId::new()) }));

        let person_usecase = UsersUseCase::creation(Arc::new(persons_repository_mock), time_helper);

        let result = match person_usecase.adding(req.name, req.email).await {
            Ok(inserted_id) => inserted_id,
            Err(_) => panic!("adding_test failed"),
        };

        assert_ne!(result, ObjectId::new());
    }
}
