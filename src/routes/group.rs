use std::{collections::HashMap, sync::Arc};
use axum::{extract::{self, Query, State}, http::StatusCode, response::IntoResponse, routing::{get, post, put}, Extension, Form, Json, Router};
use gauth::{validate_token, Auth};
use serde::Deserialize;
use serde_json::json;

use crate::{state::ServerState, utils::queries::fetch_groups_for_user};
use crate::utils::types::{CreateGroupForm, AddUsersForm, RemoveUserForm, EditPictureForm};



pub fn router() -> Router<Arc<ServerState>> {
    Router::new()
        .route("/create", post(create_group_chat))
        .route("/get", get(get_groups))
        .route("/get-users", get(get_users_in_group))
        .route("/add-users", post(add_users_to_group))
        .route("/remove-user", post(remove_user_from_group))
        .route("/edit-picture", put(edit_group_picture))
}


async fn create_group_chat(
    Extension(auth): Extension<Arc<Auth>>,
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

    let group = sqlx::query!(
        r#"
        INSERT INTO groups (name)
        VALUES ($1)
        RETURNING id
        "#,
        form.group_name
    ).fetch_one(&state.db).await;
    
    let group = match group {
        Ok(group) => group,
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
        let result = sqlx::query!(
            r#"
            INSERT INTO group_members (group_id, user_id)
            VALUES ($1, $2)
            "#,
            group.id,
            member_id
        )
        .execute(&state.db)
        .await;

        match result {
            Ok(_) => {}
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"message": "Internal Server Error"})),
                );
            }
        }
    }
    (StatusCode::OK, Json(json!({"message": "Group Created", "group_id": group.id})))

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
        Some(group_id) => group_id,
    };
    let group_id = group_id.parse::<i32>().unwrap();
    let users = sqlx::query!(
        r#"
        SELECT *
        FROM users u
        JOIN group_members gm ON u.id = gm.user_id
        WHERE gm.group_id = $1
        "#,
        group_id
    ).fetch_all(&state.db).await;

    match users {
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
    Extension(auth): Extension<Arc<Auth>>,
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

    let users = sqlx::query!(
        r#"
        SELECT *
        FROM users u
        JOIN group_members gm ON u.id = gm.user_id
        WHERE gm.group_id = $1
        "#,
        form.group_id
    ).fetch_all(&state.db).await;

    match users {
        Ok(users) => {
            let users_data = users.iter().map(|u| {
                json!({
                    "username": u.username,
                    "profile_picture": u.profile_picture,
                    "id": u.id,
                })
            }).collect::<Vec<_>>();

            let mut user_in_group = false;
            for user in users_data.iter() {
                if user["id"] == user_id {
                    user_in_group = true;
                    break;
                }
            }
            if !user_in_group {
                return (StatusCode::UNAUTHORIZED,Json(json!({ "error": "User not in group" }))).into_response()
            }
        }
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR,Json(json!({ "error": "Failed to fetch users" }))).into_response()
        }
    };

    for member_id in form.new_member_ids {
        let result = sqlx::query!(
            r#"
            INSERT INTO group_members (group_id, user_id)
            VALUES ($1, $2)
            "#,
            form.group_id,
            member_id
        )
       .execute(&state.db)
       .await;
        match result {
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
    Extension(auth): Extension<Arc<Auth>>,
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

    let users = sqlx::query!(
        r#"
        SELECT *
        FROM users u
        JOIN group_members gm ON u.id = gm.user_id
        WHERE gm.group_id = $1
        "#,
        form.group_id
    ).fetch_all(&state.db).await;

    match users {
        Ok(users) => {
            let users_data = users.iter().map(|u| {
                json!({
                    "username": u.username,
                    "profile_picture": u.profile_picture,
                    "id": u.id,
                })
            }).collect::<Vec<_>>();

            let mut user_in_group = false;
            for user in users_data.iter() {
                if user["id"] == user_id {
                    user_in_group = true;
                    break;
                }
            }
            if !user_in_group {
                return (StatusCode::UNAUTHORIZED,Json(json!({ "error": "User not in group" }))).into_response()
            }
        }
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR,Json(json!({ "error": "Failed to fetch users" }))).into_response()
        }
    };


        let result = sqlx::query!(
            r#"
            DELETE
            FROM group_members
            WHERE group_id = $1 AND user_id = $2
            "#,
            form.group_id,
            form.remove_id
        )
       .execute(&state.db)
       .await;
        match result {
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
    Extension(auth): Extension<Arc<Auth>>,
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

    let users = sqlx::query!(
        r#"
        SELECT *
        FROM users u
        JOIN group_members gm ON u.id = gm.user_id
        WHERE gm.group_id = $1
        "#,
        form.group_id
    ).fetch_all(&state.db).await;

    match users {
        Ok(users) => {
            let users_data = users.iter().map(|u| {
                json!({
                    "username": u.username,
                    "profile_picture": u.profile_picture,
                    "id": u.id,
                })
            }).collect::<Vec<_>>();

            let mut user_in_group = false;
            for user in users_data.iter() {
                if user["id"] == user_id {
                    user_in_group = true;
                    break;
                }
            }
            if !user_in_group {
                return (StatusCode::UNAUTHORIZED,Json(json!({ "error": "User not in group" }))).into_response()
            }
        }
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR,Json(json!({ "error": "Failed to fetch users" }))).into_response()
        }
    };


        let result = sqlx::query!(
            r#"
            UPDATE groups
            SET profile_picture = $1
            WHERE id = $2
            "#,
            form.picture_url,
            form.group_id
        )
       .execute(&state.db)
       .await;
        match result {
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

