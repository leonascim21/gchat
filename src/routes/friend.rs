use std::{collections::HashMap, sync::Arc};
use axum::{extract::{Query, State}, http::StatusCode, response::IntoResponse, routing::{get, post}, Extension, Form, Json, Router};
use gauth::{validate_token, Auth};
use serde_json::json;

use crate::utils::{queries::{add_group_member, create_friendship, create_group, delete_friend_request, delete_friendship, delete_group, fetch_dm_id, fetch_friend_request}, types::{FriendForm, FriendRequestForm}};
use crate::utils::queries::{fetch_friends_for_user, create_friend_request, fetch_incoming_requests, fetch_outgoing_requests};

use crate::state::ServerState;



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
    
    match fetch_friends_for_user(user_id, &state.db).await {
        Ok(friendships) => {
            let friendship_data = friendships.iter().map(|f| {
                json!({
                    "friend_id": f.id,
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

    let outgoing_requests = match fetch_outgoing_requests(user_id, &state.db).await {
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

    let incoming_requests = match fetch_incoming_requests(user_id, &state.db).await {
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
    let user_id = match validate_token(&form.token, jwt_key).await {
        Ok(claims) => claims.sub.parse::<i32>().unwrap(),
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Unauthorized"})),
            );
        }
    };
    let receiver =  match auth.get_user_by_username(form.receiver_username).await {
        Ok(user) => if user.is_some() { 
            let receving_user_id = match user.as_ref().unwrap().id {
                Some(id) => id,
                None => {
                    return (StatusCode::BAD_REQUEST, Json(json!({"message": "User not found" }))); 
                }
            }; 
            if receving_user_id == user_id {
                return (StatusCode::BAD_REQUEST, Json(json!({"message": "Cannot add yourself!" }))); 
            } else if fetch_friends_for_user(user_id, &state.db).await.unwrap().iter().any(|friend| friend.id == receving_user_id) {
                return (StatusCode::BAD_REQUEST, Json(json!({"message": "User is already your friend" }))); 
            } else {
                user
            }
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
    match create_friend_request(user_id, receiver.unwrap().id.unwrap(), &state.db).await {
        Ok(request) => {
            let friend_request = json!(request);
            (StatusCode::OK, Json(json!({"message": "Friend request sent successfully", "friend_request": friend_request})))
    },
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"message": "Failed to send friend request" }))),
    }
}

async fn accept_friend_request(
    State(state): State<Arc<ServerState>>,
    Form(form): Form<FriendForm>,
) -> impl IntoResponse {
    let jwt_key = std::env::var("JWT_KEY").expect("JWT_KEY must be set");
    // Validate the token.
    let user_id = match validate_token(&form.token, jwt_key).await {
        Ok(claims) => claims.sub.parse::<i32>().unwrap(),
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Unauthorized"})),
            );
        }
    };

    //FETCHING FRIEND REQUEST TO CHECK IF IT EXISTS BEFORE CREATING FRIENDSHIP
    match fetch_friend_request(form.user_id, user_id, &state.db).await {
        Ok(_) => {},
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal server error, please try again"})),
            );
        }
    };

    let added_friend = create_friendship(user_id, form.user_id, &state.db).await;
    let reverse_friend = create_friendship(form.user_id, user_id, &state.db).await;
    let delete_request = delete_friend_request(form.user_id, user_id, &state.db).await;

    let create_dm = create_group("DM".to_string(), 2, &state.db).await;
    match create_dm {
        Ok(group_id) => {
            let member1 = add_group_member(user_id, group_id, &state.db).await;
            let member2 = add_group_member(form.user_id, group_id, &state.db).await;
            match (member1, member2) {
                (Ok(_), Ok(_)) => {},
                _ => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"message": "Failed to add members to DM"})),
                    );
                }
            }
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to create DM"})),
            );
        }
    }
    
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
    Form(form): Form<FriendForm>,
) -> impl IntoResponse {
    let jwt_key = std::env::var("JWT_KEY").expect("JWT_KEY must be set");
    // Validate the token.
    let user_id = match validate_token(&form.token, jwt_key).await {
        Ok(claims) => claims.sub.parse::<i32>().unwrap(),
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Unauthorized"})),
            );
        }
    };

    match delete_friend_request(user_id, form.user_id, &state.db).await {
        Ok(_) => {
            (StatusCode::OK, Json(json!({"message": "Friend request canceled"})))
        },
        Err(_)=> {
            (StatusCode::INTERNAL_SERVER_ERROR, 
             Json(json!({"message": "Failed to cancel friend request"})))
        }
    }

}

async fn deny_friend_request(
    State(state): State<Arc<ServerState>>,
    Form(form): Form<FriendForm>,
) -> impl IntoResponse {
    let jwt_key = std::env::var("JWT_KEY").expect("JWT_KEY must be set");
    // Validate the token.
    let user_id = match validate_token(&form.token, jwt_key).await {
        Ok(claims) => claims.sub.parse::<i32>().unwrap(),
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Unauthorized"})),
            );
        }
    };

    match delete_friend_request(form.user_id, user_id, &state.db).await {
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
    Form(form): Form<FriendForm>
) -> impl IntoResponse  {
    let jwt_key = std::env::var("JWT_KEY").expect("JWT_KEY must be set");
    // Validate the token.
    let user_id = match validate_token(&form.token, jwt_key).await {
        Ok(claims) => claims.sub.parse::<i32>().unwrap(),
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"message": "Unauthorized"})),
            );
        }
    };

    let remove_friendship = delete_friendship(user_id, form.user_id, &state.db).await;
    let remove_reverse_friendship = delete_friendship(form.user_id, user_id, &state.db).await;

    match fetch_dm_id(user_id, form.user_id, &state.db).await {
        Ok(dm_id) => {
            match delete_group(dm_id, &state.db).await {
                Ok(_) => {},
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"message": "Failed to delete DM"})),
                    );
                }
            };
        },
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to fetch DM ID"})),
            );
        }
    };

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