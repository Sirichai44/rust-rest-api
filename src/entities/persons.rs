use crate::time_helper::IntoTimerHelperShared;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub id: mongodb::bson::oid::ObjectId,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Person {
    pub fn new(name: String, email: String, t: IntoTimerHelperShared) -> Self {
        let mg_id = mongodb::bson::oid::ObjectId::new();

        Self {
            id: mg_id,
            name,
            email,
            created_at: t.now(),
            updated_at: t.now(),
        }
    }
}
