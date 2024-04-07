use axum::{
    extract::Extension, routing::{delete, get, post, put}, Router
};
use mongodb::Client;
use std::sync::Arc;

use crate::controllers;

pub fn route(client: Arc<Client>) -> Router {
    Router::new()
        .route("/", get(|| async { "Selamat datang di web sevice RUST!! 🦀" }))
        .route("/users", get(controllers::get_all_user))
        .route("/user", post(controllers::add_user))
        .route("/user/:id", get(controllers::get_user))
        .route("/user/:id", put(controllers::update_user))
        .route("/user/:id", delete(controllers::delete_user))
        .layer(Extension(client))
}
