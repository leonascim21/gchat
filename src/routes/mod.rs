use std::sync::Arc;
use axum::Router;
use crate::state::ServerState;

pub mod auth;
pub mod friend;

pub fn app_routes() -> Router<Arc<ServerState>> {
    Router::new()
    .nest("/friend", friend::router())
}