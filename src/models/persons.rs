use serde::Deserialize;

use crate::{entities::persons::Person as PersonsEntity, time_helper::IntoTimerHelperShared};

#[derive(Deserialize)]
pub struct Person {
    pub name: String,
    pub email: String,
}

impl Person {
    pub fn to_entity(&self, t: IntoTimerHelperShared) -> PersonsEntity {
        PersonsEntity::new(self.name.to_string(), self.email.to_string(), t)
    }
}
