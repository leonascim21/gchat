use std::{collections::HashMap, sync::Arc};
use axum::{extract::{self, Query, State}, http::StatusCode, response::IntoResponse, routing::{get, post, put}, Form, Json, Router};
use gauth::validate_token;
use serde_json::json;

use crate::{state::ServerState, utils::queries::{change_group_picture, fetch_messages, is_user_in_group, remove_group_member}};
use crate::utils::types::{CreateGroupForm, AddUsersForm, RemoveUserForm, EditPictureForm};
use crate::utils::queries::{fetch_group_members, fetch_groups_for_user, add_group_member, create_group};



pub fn router() -> Router<Arc<ServerState>> {
    Router::new()
        .route("/create", post(create_group_chat))
        .route("/get", get(get_groups))
        .route("/get-users", get(get_users_in_group))
        .route("/add-users", post(add_users_to_group))
        .route("/remove-user", post(remove_user_from_group))
        .route("/edit-picture", put(edit_group_picture))
        .route("/get-messages", get(get_group_messages))
}


async fn create_group_chat(
    State(state): State<Arc<ServerState>>,
    extract::Json(form): extract::Json<CreateGroupForm>
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
    
    let group_id = match create_group(form.group_name, &state.db).await {
        Ok(group_id) => group_id,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal Server Error"})),
            );
        }
    };

    let mut member_ids = form.member_ids;
    //add self to member ids array
    member_ids.push(claims.sub.parse::<i32>().unwrap());
    
    for member_id in member_ids {
        match add_group_member(member_id, group_id, &state.db).await {
            Ok(_) => {}
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"message": "Internal Server Error"})),
                );
            }
        }
    }
    (StatusCode::OK, Json(json!({"message": "Group Created", "group_id": group_id})))

}

async fn get_groups(
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

    match fetch_groups_for_user(user_id, &state.db).await {
        Ok(groups) => {
            let groups_data = groups.iter().map(|g| {
                json!({
                    "name": g.name,
                    "profile_picture": g.profile_picture,
                    "id": g.id,
                })
            }).collect::<Vec<_>>();
            (StatusCode::OK, Json(groups_data)).into_response()
        }
        Err(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR,Json(json!({ "error": "Failed to fetch friendships" }))).into_response()
        }
    }
}

async fn get_users_in_group(    
    State(state): State<Arc<ServerState>>,
    Query(params): Query<HashMap<String, String>>
) -> impl IntoResponse  {
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
    let group_id = match params.get("group_id") {
        None => {
            return (StatusCode::BAD_REQUEST, Json(json!({ "error": "Missing group_id" }))).into_response()
        }
        Some(group_id) => group_id.parse::<i32>().unwrap(),
    };

    match fetch_group_members(group_id, &state.db).await {
        Ok(users) => {
            let users_data = users.iter().map(|u| {
                json!({
                    "username": u.username,
                    "profile_picture": u.profile_picture,
                    "friend_id": u.id,
                })
            }).collect::<Vec<_>>();

            let mut user_in_group = false;
            for user in users_data.iter() {
                if user["friend_id"] == user_id {
                    user_in_group = true;
                    break;
                }
            }
            if !user_in_group {
                return (StatusCode::UNAUTHORIZED,Json(json!({ "error": "User not in group" }))).into_response()
            }

            (StatusCode::OK, Json(users_data)).into_response()
        }
        Err(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR,Json(json!({ "error": "Failed to fetch users" }))).into_response()
        }
    }
}


async fn add_users_to_group(
    State(state): State<Arc<ServerState>>,
    extract::Json(form): extract::Json<AddUsersForm>
) -> impl IntoResponse {
    let jwt_key = std::env::var("JWT_KEY").expect("JWT_KEY must be set");
    let user_id = match validate_token(&form.token, jwt_key).await {
        Ok(claims) => claims.sub.parse::<i32>().unwrap(),
        Err(_) => {
            return (StatusCode::UNAUTHORIZED,Json(json!({ "error": "Invalid token" }))).into_response()
        }
    };

    match is_user_in_group(user_id, form.group_id, &state.db).await {
        Ok(_) => {}
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR,Json(json!({ "error": "Unauthorized" }))).into_response()
        }
    };

    for member_id in form.new_member_ids {
        match add_group_member(member_id, form.group_id, &state.db).await {
            Ok(_) => {}
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"message": "Internal Server Error"}))).into_response();
            }
        }
    }

    return (StatusCode::OK, Json(json!({"message": "Users Added"}))).into_response();

  
}


async fn remove_user_from_group(
    State(state): State<Arc<ServerState>>,
    Form(form): Form<RemoveUserForm>,
) -> impl IntoResponse {
    let jwt_key = std::env::var("JWT_KEY").expect("JWT_KEY must be set");
    let user_id = match validate_token(&form.token, jwt_key).await {
        Ok(claims) => claims.sub.parse::<i32>().unwrap(),
        Err(_) => {
            return (StatusCode::UNAUTHORIZED,Json(json!({ "error": "Invalid token" }))).into_response()
        }
    };

    match is_user_in_group(user_id, form.group_id, &state.db).await {
        Ok(_) => {}
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR,Json(json!({ "error": "Unauthorized" }))).into_response()
        }
    };

    match remove_group_member(form.remove_id, form.group_id, &state.db).await {
        Ok(_) => {
            return (StatusCode::OK, Json(json!({"message": "User Removed"}))).into_response();
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to remove user"}))).into_response();
        }
    }
}

async fn edit_group_picture(
    State(state): State<Arc<ServerState>>,
    Form(form): Form<EditPictureForm>,
) -> impl IntoResponse {
    let jwt_key = std::env::var("JWT_KEY").expect("JWT_KEY must be set");
    let user_id = match validate_token(&form.token, jwt_key).await {
        Ok(claims) => claims.sub.parse::<i32>().unwrap(),
        Err(_) => {
            return (StatusCode::UNAUTHORIZED,Json(json!({ "error": "Invalid token" }))).into_response()
        }
    };

    match is_user_in_group(user_id, form.group_id, &state.db).await {
        Ok(_) => {}
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR,Json(json!({ "error": "Unauthorized" }))).into_response()
        }
    };

    match change_group_picture(form.group_id, form.picture_url, &state.db).await {
        Ok(_) => {
            return (StatusCode::OK, Json(json!({"message": "Picture Updated"}))).into_response();
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to update picture"}))).into_response();
        }
    }
}

async fn get_group_messages(
    State(state): State<Arc<ServerState>>,
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

    let group_id = params.get("group_id");
    if group_id.is_none() {

    }
    let group_id = match group_id {
        Some(group_id) => group_id.parse::<i32>().unwrap(),
        None => {
            return (
                StatusCode::UNAUTHORIZED, Json(json!({ "error": "Missing group id" })),
            ).into_response();
        }
    };

    let jwt_key = std::env::var("JWT_KEY").expect("JWT_KEY must be set");
    let user_id = match validate_token(token.unwrap(), jwt_key).await {
        Ok(claims) => claims.sub.parse::<i32>().unwrap(),
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "Invalid token" })),
            )
            .into_response()
        }
    };

    match is_user_in_group(user_id, group_id, &state.db).await {
        Ok(_) => {},
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "Invalid token" })),
            )
            .into_response()
        }
    }

    match fetch_messages(group_id, &state.db).await {
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
