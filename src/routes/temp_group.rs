use std::{collections::HashMap, sync::Arc};
use axum::{extract::{self, Query, State}, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router};
use gauth::validate_token;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use bcrypt::verify;


use crate::{state::ServerState, utils::queries::{delete_group, fetch_temp_chats_for_user}};
use crate::utils::types::CreateTempGroupForm;
use crate::utils::queries::{create_temp_chat, fetch_messages, fetch_temp_chat};

pub fn router() -> Router<Arc<ServerState>> {
    Router::new()
        .route("/get-messages", get(get_group_messages))
        .route("/get-group-info", get(get_group_info))
        .route("/has-password", get(has_password))
        .route("/create", post(create_group_chat))
        .route("/get", get(get_temp_groups_for_user))
}


async fn get_temp_groups_for_user (    
    State(state): State<Arc<ServerState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let token = match params.get("token") {
        None => {
            return (StatusCode::UNAUTHORIZED, Json(json!({ "error": "Missing token" }))).into_response()
        }
        Some(token) => token,
    };
    let jwt_key = std::env::var("JWT_KEY").expect("JWT_KEY must be set");
    let user_id = match validate_token(token, jwt_key).await {
        Ok(claims) => claims.sub.parse::<i32>().unwrap(),
        Err(_) => {
            return (StatusCode::UNAUTHORIZED,Json(json!({ "error": "Invalid token" }))).into_response()
        }
    };

    match fetch_temp_chats_for_user(user_id, &state.db).await {
        Ok(temp_chats) => {
            
            let mut valid_temp_chats = Vec::new();
            for chat in temp_chats {
                match check_end_date(chat.end_date, chat.group_id, &state.db).await {
                    Ok(_) => {
                        valid_temp_chats.push(chat);
                    },
                    Err(_) => {}
                }
            }
            let temp_chats = valid_temp_chats;
            
            let temp_chats_data = temp_chats.iter().map(|t| {
                json!({
                    "temp_chat_key": t.temp_chat_key,
                    "group_id": t.group_id,
                    "end_date": t.end_date.to_rfc3339(),
                    "name": t.name,
                })
            }).collect::<Vec<_>>();
            
            
            (StatusCode::OK, Json(temp_chats_data)).into_response()
        },
        Err(err) => {
            eprintln!("Database error: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Failed to fetch temp chats" }))).into_response()
        }
    }
}

async fn create_group_chat(
    State(state): State<Arc<ServerState>>,
    extract::Json(form): extract::Json<CreateTempGroupForm>
) -> impl IntoResponse {

    let jwt_key = std::env::var("JWT_KEY").expect("JWT_KEY must be set");
    let user_id = match validate_token(&form.token, jwt_key).await {
        Ok(claims) => claims.sub.parse::<i32>().unwrap(),
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "Invalid token" })),
            )
        }
    };

    let unique_id = Uuid::new_v4().to_string();
    let end_date = match form.end_date.parse::<DateTime<Utc>>() {
        Ok(end_date) => end_date,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"message": "Invalid end date format"})),
            );
        }
    };
 match create_temp_chat(unique_id, form.group_name, end_date, form.password, user_id, &state.db).await {
        Ok(result) => {
            let chat_key = result.0;
            let group_id = result.1;
            return (StatusCode::OK, 
                Json(json!({"message": "Group Created", "chat_key": chat_key, "group_id": group_id})))},
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal Server Error"})),
            );
        }
    };

}   

async fn has_password (    
    State(state): State<Arc<ServerState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let temp_chat_key = params.get("temp");
    let temp_chat_key = match temp_chat_key {
        Some(temp_chat_key) => temp_chat_key.parse::<String>().unwrap(),
        None => {
            return (
                StatusCode::UNAUTHORIZED, Json(json!({ "error": "Missing chat key" })),
            ).into_response();
        }
    };

    let temp_chat_info = match fetch_temp_chat(temp_chat_key, &state.db).await {
        Ok(temp_chat_info) => temp_chat_info,
        Err(err) => {
            eprintln!("Database error: {}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to fetch chat info" })),
            ).into_response();
        }
    };

    match check_end_date(temp_chat_info.end_date, temp_chat_info.group_id, &state.db).await {
        Ok(_) => {}
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal Server Error"})),
            ).into_response();
        }
    }
    
    if temp_chat_info.password.is_some() {
        return (
            StatusCode::OK, Json(json!({ "has_password": true })),
        ).into_response();
    }
    return (
        StatusCode::OK, Json(json!({ "has_password": false })),
    ).into_response();

}

async fn get_group_info(
    State(state): State<Arc<ServerState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {

    let temp_chat_key = params.get("temp");
    let temp_chat_key = match temp_chat_key {
        Some(temp_chat_key) => temp_chat_key.parse::<String>().unwrap(),
        None => {
            return (
                StatusCode::UNAUTHORIZED, Json(json!({ "error": "Missing chat key" })),
            ).into_response();
        }
    };

    match fetch_temp_chat(temp_chat_key, &state.db).await {
        Ok(temp_chat_info) => {

            match check_end_date(temp_chat_info.end_date, temp_chat_info.group_id, &state.db).await {
                Ok(_) => {}
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"message": "Internal Server Error"})),
                    ).into_response();
                } 
            }

            if temp_chat_info.password.is_some() {
                let password = params.get("password");
                let password = match password {
                    Some(password) => password.parse::<String>().unwrap(),
                    None => {
                        return (
                            StatusCode::UNAUTHORIZED, Json(json!({ "error": "Unauthorized" })),
                        ).into_response();
                    }
                };
                if !(verify(password, &temp_chat_info.password.unwrap()).unwrap_or(false)) {
                    return (
                        StatusCode::UNAUTHORIZED, Json(json!({ "error": "Unauthorized" })),
                    ).into_response();
                }
            }
            return (
                StatusCode::OK, Json(json!({
                    "chat_key": temp_chat_info.temp_chat_key,
                    "group_id": temp_chat_info.group_id,
                    "end_date": temp_chat_info.end_date.to_rfc3339(),
                    "name": temp_chat_info.name,
                })),
            ).into_response();
        },
        Err(err) => {
            eprintln!("Database error: {}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to fetch chat info" })),
            ).into_response();
        }
    };
}


async fn get_group_messages(
    State(state): State<Arc<ServerState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {

    let temp_chat_key = params.get("temp");
    let temp_chat_key = match temp_chat_key {
        Some(temp_chat_key) => temp_chat_key.parse::<String>().unwrap(),
        None => {
            return (
                StatusCode::UNAUTHORIZED, Json(json!({ "error": "Missing chat key" })),
            ).into_response();
        }
    };

    let temp_chat_info = match fetch_temp_chat(temp_chat_key, &state.db).await {
        Ok(temp_chat_info) => temp_chat_info,
        Err(err) => {
            eprintln!("Database error: {}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to fetch chat info" })),
            ).into_response();
        }
    };

    match check_end_date(temp_chat_info.end_date, temp_chat_info.group_id, &state.db).await {
        Ok(_) => {}
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal Server Error"})),
            ).into_response();
        }
    }
    
    if temp_chat_info.password.is_some() {
        let password = params.get("password");
        let password = match password {
            Some(password) => password.parse::<String>().unwrap(),
            None => {
                return (
                    StatusCode::UNAUTHORIZED, Json(json!({ "error": "Unauthorized" })),
                ).into_response();
            }
        };
        if !(verify(password, &temp_chat_info.password.unwrap()).unwrap_or(false)) {
            return (
                StatusCode::UNAUTHORIZED, Json(json!({ "error": "Unauthorized" })),
            ).into_response();
        }
    }

    match fetch_messages(temp_chat_info.group_id, &state.db).await {
        Ok(messages) => {
            let message_data = messages.iter().map(|m| {
                json!({
                    "id": m.id,
                    "content": m.content,
                    "user_id": m.user_id,
                    "username": m.username,
                    "timestamp": m.timestamp.to_rfc3339(),
                    "profile_picture": m.profile_picture
                })
            }).collect::<Vec<_>>();
            
            (StatusCode::OK, Json(message_data)).into_response()
        },
        Err(err) => {
            eprintln!("Database error: {}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to fetch messages" })),
            ).into_response()
        }
    }
}

pub async fn check_end_date(end_date: DateTime<Utc>, group_id: i32, db: &PgPool) -> Result<(), ()> {
    let now = Utc::now();
    if end_date > now {
        return Ok(());
    }
    match delete_group(group_id, db).await {
        Ok(_) => {
            return Err(());
        }
        Err(_) => {
            return Err(());
        }
    }
}