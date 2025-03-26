use std::{collections::HashMap, sync::Arc};
use axum::{extract::{self, Query, State}, http::StatusCode, response::IntoResponse, routing::{get, post}, Extension, Form, Json, Router};
use gauth::{validate_token, Auth};
use serde::Deserialize;
use serde_json::json;

use crate::state::ServerState;

#[derive(Deserialize)]
struct CreateGroupForm {
    token: String,
    #[serde(rename = "groupName")]
    group_name: String,
    #[serde(rename = "memberIds")]
    member_ids: Vec<i32>,
}

pub fn router() -> Router<Arc<ServerState>> {
    Router::new()
        .route("/create", post(create_group_chat))
        .route("/get", get(get_groups))
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
    (StatusCode::OK, Json(json!({"message": "Group Created"})))

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

    let groups = sqlx::query!(
        r#"
        SELECT *
        FROM groups g
        JOIN group_members gm ON g.id = gm.group_id
        WHERE gm.user_id = $1
        "#,
        user_id 
    )
  .fetch_all(&state.db)
  .await;
    match groups {
        Ok(groups) => {
            println!("{:?}", groups);
            let groups_data = groups.iter().map(|g| {
                json!({
                    "name": g.name,
                    "profile_picture": g.profile_picture,
                    "id": g.id,
                })
            }).collect::<Vec<_>>();
            (StatusCode::OK, Json(groups_data)).into_response()
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
