use axum::{
    body::Body,
    routing::get,
    response::Html,
    Router,
};
use serde_json::{Value, json};

let app = Router::new()
    .route("/login", get(Html("Hey there!")));
