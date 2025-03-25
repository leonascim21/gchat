use std::{collections::HashMap, sync::Arc};
use axum::{extract::{Query, State}, http::StatusCode, response::IntoResponse, routing::{get, post}, Extension, Form, Json, Router};
use gauth::{validate_token, Auth};
use serde::Deserialize;
use serde_json::json;

use crate::state::ServerState;

#[derive(Deserialize)]
struct FriendRequestForm {
    #[serde(rename = "receiverUsername")]
    receiver_username: String,
    token: String,
}

#[derive(Deserialize)]
struct AcceptFriendRequestForm {
    #[serde(rename = "userId")]
    user_id: i32,
    token: String,
}

#[derive(Deserialize)]
struct CancelFriendRequestForm {
    #[serde(rename = "userId")]
    user_id: i32,
    token: String,
}

#[derive(Deserialize)]
struct DenyFriendRequestForm {
    #[serde(rename = "userId")]
    user_id: i32,
    token: String,
}

#[derive(Deserialize)]
struct RemoveFriendForm {
    #[serde(rename = "userId")]
    user_id: i32,
    token: String,
}

pub fn router() -> Router<Arc<ServerState>> {
    Router::new()
        .route("/get", get(get_friendships))
        .route("/delete", post(remove_friendship))
        .route("/send-request", post(send_friend_request))
        .route("/get-requests", get(get_friend_requests))
        .route("/accept-request", post(accept_friend_request))
        .route("/cancel-request", post(cancel_friend_request))
        .route("/deny-request", post(deny_friend_request))
}

async fn get_friendships(
    State(state): State<Arc<ServerState>>,
    Query(params): Query<HashMap<String, String>>
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

    let friendship = sqlx::query!(
        r#"
        SELECT f.friend_id, u.username 
        FROM friendships f 
        JOIN users u ON f.friend_id = u.id 
        WHERE f.user_id = $1
        "#,
        user_id
    )
  .fetch_all(&state.db)
  .await;
    match friendship {
        Ok(friendships) => {
            let friendship_data = friendships.iter().map(|f| {
                json!({
                    "friend_id": f.friend_id,
                    "username": f.username,
                })
            }).collect::<Vec<_>>();
            (StatusCode::OK, Json(friendship_data)).into_response()
        }
        Err(err) => {
            eprintln!("Database error: {}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to fetch friendships" })),
            ).into_response()
        }
    }
}


async fn get_friend_requests(
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

    let outgoing_result = sqlx::query!(
        r#"
        SELECT fr.sender_id, fr.receiver_id, u.username
        FROM friend_requests fr
        JOIN users u ON fr.receiver_id = u.id
        WHERE fr.sender_id = $1
        "#,
        user_id
    )
    .fetch_all(&state.db)
    .await;

    let outgoing_requests = match outgoing_result {
        Ok(requests) => requests
            .iter()
            .map(|r| {
                json!({
                    "sender_id": r.sender_id,
                    "receiver_id": r.receiver_id,
                    "username": r.username,
                })
            })
            .collect::<Vec<_>>(),
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR,Json(json!({ "error": "Failed to fetch friend requests" }))).into_response()
        }
    };

    let incoming_result = sqlx::query!(
        r#"
        SELECT fr.sender_id, fr.receiver_id, u.username
        FROM friend_requests fr
        JOIN users u ON fr.sender_id = u.id
        WHERE fr.receiver_id = $1
        "#,
        user_id
    )
    .fetch_all(&state.db)
    .await;

    let incoming_requests = match incoming_result {
        Ok(requests) => requests
            .iter()
            .map(|r| {
                json!({
                    "sender_id": r.sender_id,
                    "receiver_id": r.receiver_id,
                    "username": r.username,
                })
            })
            .collect::<Vec<_>>(),
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR,Json(json!({ "error": "Failed to fetch friend requests" }))).into_response()
        }
    };

    (StatusCode::OK,Json(json!({"outgoing": outgoing_requests,"incoming": incoming_requests }))).into_response()
}

async fn send_friend_request(
    Extension(auth): Extension<Arc<Auth>>,
    State(state): State<Arc<ServerState>>,
    Form(form): Form<FriendRequestForm>,
) -> impl IntoResponse {

    let jwt_key = std::env::var("JWT_KEY").expect("JWT_KEY must be set");
    // Validate the token.
    let claims = match validate_token(&form.token, jwt_key).await {
        Ok(claims) => claims,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Unauthorized"})),
            );
        }
    };
    let receiver =  match auth.get_user_by_username(form.receiver_username).await {
        Ok(user) => if user.is_some() { 
            user.unwrap() 
        } else { 
            return (StatusCode::BAD_REQUEST, Json(json!({"message": "User not found" }))); 
        },
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal server error, please try again"})),
            );
        }
    };
    let result = sqlx::query!(
        r#"
        WITH inserted AS (
            INSERT INTO friend_requests (sender_id, receiver_id)
            VALUES ($1, $2)
            RETURNING sender_id, receiver_id
        )
        SELECT i.sender_id, i.receiver_id, u.username
        FROM inserted i
        JOIN users u ON i.receiver_id = u.id
        "#,
        claims.sub.parse::<i32>().unwrap(),
        receiver.id,
    ).fetch_one(&state.db).await;
    match result {
        Ok(record) => {
            let friend_request = json!({
                "receiver_id:": record.receiver_id,
                "sender_id": record.sender_id,
                "username": record.username,
            });
            
            (StatusCode::OK, Json(json!({"message": "Friend request sent successfully", "friend_request": friend_request})))
    },
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"message": "Failed to send friend request" }))),
    }
}

async fn accept_friend_request(
    State(state): State<Arc<ServerState>>,
    Form(form): Form<AcceptFriendRequestForm>,
) -> impl IntoResponse {
    let jwt_key = std::env::var("JWT_KEY").expect("JWT_KEY must be set");
    // Validate the token.
    let claims = match validate_token(&form.token, jwt_key).await {
        Ok(claims) => claims,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Unauthorized"})),
            );
        }
    };

    let existing_request = sqlx::query!(
        r#"
        SELECT * 
        FROM friend_requests 
        WHERE sender_id = $1 AND receiver_id = $2
        "#,
        form.user_id,
        claims.sub.parse::<i32>().unwrap()
    ).fetch_one(&state.db).await;

    let existing_request = match existing_request {
        Ok(existing_request) => existing_request,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal server error, please try again"})),
            );
        }
    };
    let added_friend = sqlx::query!(
        r#"
        INSERT INTO friendships (user_id, friend_id)
        VALUES ($1, $2);
        "#,
        form.user_id,
        claims.sub.parse::<i32>().unwrap()
    ).execute(&state.db).await;

    let reverse_friend = sqlx::query!(
        r#"
        INSERT INTO friendships (user_id, friend_id)
        VALUES ($1, $2);
        "#,
        claims.sub.parse::<i32>().unwrap(),
        form.user_id,
    ).execute(&state.db).await;

    let delete_request = sqlx::query!(
        r#"
        DELETE
        FROM friend_requests 
        WHERE sender_id = $1 AND receiver_id = $2
        "#,
        form.user_id,
        claims.sub.parse::<i32>().unwrap()
    ).execute(&state.db).await;

    match (added_friend, reverse_friend, delete_request) {
        (Ok(_), Ok(_), Ok(_)) => {
            (StatusCode::OK, Json(json!({"message": "Friend request accepted"})))
        },
        _ => {
            (StatusCode::INTERNAL_SERVER_ERROR, 
             Json(json!({"message": "Failed to accept friend request"})))
        }
    }

}

async fn cancel_friend_request(
    State(state): State<Arc<ServerState>>,
    Form(form): Form<CancelFriendRequestForm>,
) -> impl IntoResponse {
    let jwt_key = std::env::var("JWT_KEY").expect("JWT_KEY must be set");
    // Validate the token.
    let claims = match validate_token(&form.token, jwt_key).await {
        Ok(claims) => claims,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Unauthorized"})),
            );
        }
    };

    let delete_request = sqlx::query!(
        r#"
        DELETE
        FROM friend_requests 
        WHERE sender_id = $1 AND receiver_id = $2
        "#,
        claims.sub.parse::<i32>().unwrap(),
        form.user_id
    ).execute(&state.db).await;

    match (delete_request) {
        (Ok(_)) => {
            (StatusCode::OK, Json(json!({"message": "Friend request canceled"})))
        },
        _ => {
            (StatusCode::INTERNAL_SERVER_ERROR, 
             Json(json!({"message": "Failed to cancel friend request"})))
        }
    }

}

async fn deny_friend_request(
    State(state): State<Arc<ServerState>>,
    Form(form): Form<DenyFriendRequestForm>,
) -> impl IntoResponse {
    let jwt_key = std::env::var("JWT_KEY").expect("JWT_KEY must be set");
    // Validate the token.
    let claims = match validate_token(&form.token, jwt_key).await {
        Ok(claims) => claims,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Unauthorized"})),
            );
        }
    };

    let deny_request = sqlx::query!(
        r#"
        DELETE
        FROM friend_requests 
        WHERE sender_id = $1 AND receiver_id = $2
        "#,
        form.user_id,
        claims.sub.parse::<i32>().unwrap()
    ).execute(&state.db).await;

    match deny_request {
        Ok(_) => {
            (StatusCode::OK, Json(json!({"message": "Friend request denied"})))
        },
        Err(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, 
             Json(json!({"message": "Failed to deny friend request"})))
        }
    }

}


async fn remove_friendship(    
    State(state): State<Arc<ServerState>>,
    Form(form): Form<RemoveFriendForm>
) -> impl IntoResponse  {
    let jwt_key = std::env::var("JWT_KEY").expect("JWT_KEY must be set");
    // Validate the token.
    let claims = match validate_token(&form.token, jwt_key).await {
        Ok(claims) => claims,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Unauthorized"})),
            );
        }
    };

    let remove_friendship = sqlx::query!(
        r#"
        DELETE
        FROM friendships 
        WHERE user_id = $1 AND friend_id = $2
        "#, 
        claims.sub.parse::<i32>().unwrap(),
        form.user_id
    ).execute(&state.db).await;

    let remove_reverse_friendship = sqlx::query!(
        r#"
        DELETE 
        FROM friendships
        WHERE user_id = $1 AND friend_id = $2
        "#, 
        form.user_id,
        claims.sub.parse::<i32>().unwrap(),
    ).execute(&state.db).await;

    match(remove_friendship, remove_reverse_friendship) {
        (Ok(_), Ok(_)) => {
            (StatusCode::OK, Json(json!({"message": "Friend removed"})))
        },
        _ => {
            (StatusCode::INTERNAL_SERVER_ERROR,
             Json(json!({"message": "Failed to remove friend"})))
        }
    }

}