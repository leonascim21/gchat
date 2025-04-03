use sqlx::PgPool;
use crate::utils::types::{Group, Friend};

pub async fn fetch_groups_for_user(user_id: i32, db: &PgPool) -> Result<Vec<Group>, sqlx::Error> {
    sqlx::query_as!(
        Group,
        r#"
        SELECT g.id, g.name, g.profile_picture
        FROM groups g
        JOIN group_members gm ON g.id = gm.group_id
        WHERE gm.user_id = $1
        "#,
        user_id
    )
    .fetch_all(db).await
}


pub async fn fetch_group_members(group_id: i32, db: &PgPool) -> Result<Vec<Friend>, sqlx::Error>{
    sqlx::query_as!(
        Friend,
        r#"
        SELECT u.id, u.username, u.profile_picture
        FROM users u
        JOIN group_members gm ON u.id = gm.user_id
        WHERE gm.group_id = $1
        "#,
        group_id
    ).fetch_all(db).await
}

pub async fn add_group_member(user_id: i32, group_id: i32, db: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO group_members (group_id, user_id)
        VALUES ($1, $2)
        "#,
        group_id,
        user_id
    )
    .execute(db)
    .await?;
    Ok(())
}