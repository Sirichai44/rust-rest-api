use axum::{http::Method, routing::post, Extension, Router};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::info;

use rust_api::{
    database, handlers::users::users_adding, repositories::user::UserRepository, setting::Setting,
    time_helper::TimerHelper, usecases::users::UsersUseCase,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let setting = match Setting::new() {
        Err(error) => {
            panic!("Failed to load setting: {}", error);
        }

        Ok(setting) => {
            info!("Setting loaded successfully");
            setting
        }
    };

    let db_mongo = match database::conn_getting(Arc::clone(&setting)).await {
        Err(error) => {
            panic!("Failed to connect to database: {}", error);
        }

        Ok(db_mongo) => {
            info!("Database connected successfully");
            db_mongo
        }
    };

    let users_repository = UserRepository::new(db_mongo);
    let timer_helper = TimerHelper::Directly.creation();
    let users_usecase =
        UsersUseCase::creation(Arc::clone(&users_repository), Arc::clone(&timer_helper));

    let app: Router = Router::new()
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                .allow_origin(Any),
        )
        .route(
            "/users",
            // post(users_adding),
            post({
                let usecase = Arc::clone(&users_usecase);
                move |body| users_adding(Extension(usecase), body)
            }),
        )
        // .with_state(Arc::clone(&users_usecase))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], setting.server.port as u16));
    let listener = TcpListener::bind(&addr).await.unwrap();

    info!("Server running on port {}", addr);
    axum::serve(listener, app).await.unwrap();
}
