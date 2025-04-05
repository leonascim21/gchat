use std::{collections::HashMap, sync::Arc};
use axum::{Extension, extract::Query, http::StatusCode, response::IntoResponse, routing::{get, post}, Form, Json, Router};
use gauth::validate_token;
use serde_json::json;
use gauth::models::{Auth, Claims, User};
use std::time::SystemTime;

use crate::utils::types::{LoginForm, RegisterForm};
use crate::state::ServerState;



pub fn router() -> Router<Arc<ServerState>> {
    Router::new()
    .route("/login", post(handle_login))
    .route("/register", post(handle_registration))
    .route("/check-token", get(check_token))
    .route("/get-user-info", get(get_user_info))
}

async fn check_token(
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let token = params.get("token");
    if token.is_none() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Missing token" })),
        )
           .into_response();
    }
    let jwt_key = std::env::var("JWT_KEY").expect("JWT_KEY must be set");
    match validate_token(token.unwrap(), jwt_key).await {
        Ok(_) => (StatusCode::OK, Json(json!({ "valid": true }))).into_response(),
        Err(_) => (StatusCode::OK, Json(json!({ "valid": false }))).into_response(),
    }
}


async fn handle_registration(
    Extension(auth): Extension<Arc<Auth>>,
    Form(form): Form<RegisterForm>,
) -> impl IntoResponse {
    // Validate passwords match
    if form.password != form.confirm_password {
        return Err((
            StatusCode::BAD_REQUEST,
            "Passwords do not match".to_string(),
        ));
    }

    // Create a new user
    let user = User {
        id: None,
        username: form.username,
        email: Some(form.email),
        profile_picture: form.profile_picture,
        password: form.password, // Note: You should hash this password
        created_at: None,
    };

    // Register the user
    match auth.register_user(user).await {
        Ok(new_user) =>  {

            let user_id = new_user
                .id
                .ok_or_else(|| (StatusCode::INTERNAL_SERVER_ERROR, "No user ID".into()))?;

            let claims = Claims {
                sub: user_id.to_string(),
                exp: (SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
                    + (60 * 60 * 24 * 7)),
            };

            let jwt_key = std::env::var("JWT_KEY").expect("JWT_KEY must be set");
            let token = jsonwebtoken::encode(
                &jsonwebtoken::Header::default(),
                &claims,
                &jsonwebtoken::EncodingKey::from_secret(jwt_key.as_bytes()),
            )
            .unwrap();

            // Return JSON with token
            Ok(Json(json!({ "token": token })))
        }
        Err(_) => Err((
            StatusCode::BAD_REQUEST,
            "Username or email already exists".to_string(),
        )),
    }
}

async fn handle_login(
    Extension(auth): Extension<Arc<Auth>>,
    Form(form): Form<LoginForm>,
) -> impl IntoResponse {
    let user = User {
        id: None,
        username: form.username,
        email: None,
        profile_picture: None,
        password: form.password,
        created_at: None,
    };

    match auth.user_login(user).await {
        Ok(Some(user)) => {
            let claims = Claims {
                sub: user.id.unwrap().to_string(),
                exp: (SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
                    + (60 * 60 * 24 * 7)),
            };

            let jwt_key = std::env::var("JWT_KEY").expect("JWT_KEY must be set");
            let token = jsonwebtoken::encode(
                &jsonwebtoken::Header::default(),
                &claims,
                &jsonwebtoken::EncodingKey::from_secret(jwt_key.as_bytes()),
            )
            .unwrap();

            // Return JSON with token
            Ok(Json(json!({ "token": token })))
        }
        Ok(None) => Err((
            StatusCode::UNAUTHORIZED,
            "No user with these credentials exists".to_string(),
        )),
        Err(_) => Err((
            StatusCode::BAD_REQUEST,
            "Username or password wrong".to_string(),
        )),
    }
}


async fn get_user_info(
    Extension(auth): Extension<Arc<Auth>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let token = params.get("token");

    if token.is_none() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Missing token" })),
        )
            .into_response();
    }

    let jwt_key = std::env::var("JWT_KEY").expect("JWT_KEY must be set");

    match validate_token(token.unwrap(), jwt_key).await {
        Ok(claims) => {
            // Get user details from DB using the user ID in the token
            match auth.get_user_by_id(claims.sub.parse::<i32>().unwrap()).await {
                Ok(Some(user)) => {
                    // Don't include password in response
                    (
                        StatusCode::OK,
                        Json(json!({
                            "id": user.id,
                            "username": user.username,
                            "email": user.email,
                            "profile_picture": user.profile_picture
                        })),
                    )
                        .into_response()
                }
                _ => (
                    StatusCode::NOT_FOUND,
                    Json(json!({ "error": "User not found" })),
                )
                    .into_response(),
            }
        }
        Err(_) => (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Invalid token" })),
        )
            .into_response(),
    }
}