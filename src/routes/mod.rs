use std::sync::Arc;
use axum::Router;
use crate::state::ServerState;

pub mod user;
pub mod friend;
pub mod group;
pub mod temp_group;

pub fn app_routes() -> Router<Arc<ServerState>> {
    Router::new()
    .nest("/friend", friend::router())
    .nest("/group", group::router())
    .nest("/user", user::router())
    .nest("/temp-group", temp_group::router())
}