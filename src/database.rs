use std::sync::Arc;

use crate::setting::Setting;
use bson::{doc, Document};
use mongodb::{options::ClientOptions, Client, Collection, Database};
use tracing::info;

pub async fn conn_getting(setting: Arc<Setting>) -> Result<Database, mongodb::error::Error> {
    let url = setting.database.url_getting();

    info!(
        "Connecting to database {}:{}",
        setting.database.host, setting.database.port
    );
    let client_options = ClientOptions::parse(&url).await?;
    let client = Client::with_options(client_options)?;

    let db = client.database("rust_api");

    initialize_database(&db).await?;
    Ok(db)
}

async fn initialize_database(db: &Database) -> Result<(), mongodb::error::Error> {
    let collection: Collection<Document> = db.collection("users");

    if collection.count_documents(None, None).await? == 0 {
        let initial_data = vec![doc! { "name": "Test", "email": "test@mail.com" }];
        collection.insert_many(initial_data, None).await?;
    }

    Ok(())
}
