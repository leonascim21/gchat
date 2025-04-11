use sqlx::PgPool;
use crate::utils::types::{Group, Friend, Message};

pub async fn fetch_groups_for_user(user_id: i32, db: &PgPool) -> Result<Vec<Group>, sqlx::Error> {
    sqlx::query_as!(
        Group,
        r#"
        SELECT g.id, g.name, g.profile_picture, g.group_type
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

pub async fn create_group(group_name: String, group_type: i32, db: &PgPool) -> Result<i32, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        INSERT INTO groups (name, group_type)
        VALUES ($1, $2)
        RETURNING id
        "#,
        group_name, group_type
    ).fetch_one(db).await?;

    Ok(result.id)
}

pub async fn remove_group_member(user_id: i32, group_id: i32, db: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE
        FROM group_members
        WHERE group_id = $1 AND user_id = $2
        "#,
        group_id,
        user_id
    )
    .execute(db)
    .await?;
    Ok(())
}

pub async fn is_user_in_group(user_id: i32, group_id: i32, db: &PgPool) -> Result<(), sqlx::Error> {
    let members =  fetch_group_members(group_id, db).await?;

        for member in members.iter() {
            if member.id == user_id {
                return Ok(());
            }
        }

        Err(sqlx::Error::RowNotFound)
}

pub async fn change_group_picture(group_id: i32, picture_url: String, db: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE groups
        SET profile_picture = $1
        WHERE id = $2
        "#,
        picture_url,
        group_id
    )
    .execute(db)
    .await?;
    Ok(())
}

pub async fn fetch_messages(group_id: i32, db: &PgPool) -> Result<Vec<Message>, sqlx::Error> {
    sqlx::query_as!(
        Message,
        r#"
        SELECT m.id, m.content, m.user_id, m.timestamp, m.group_id, u.username, u.profile_picture 
        FROM messages m
        JOIN users u ON m.user_id = u.id
        WHERE m.group_id = $1
        ORDER BY m.timestamp
        "#,
        group_id
    )
    .fetch_all(db).await
}

pub async fn insert_message_in_db(user_id: i32, group_id:i32, content: String, db: &PgPool) 
-> Result<Message, sqlx::Error> {
    sqlx::query_as!(
        Message,
        r#"
        WITH inserted AS (
          INSERT INTO messages (user_id, content, group_id)
          VALUES ($1, $2, $3)
          RETURNING id, user_id, content, timestamp, group_id
        )
        SELECT i.id, i.user_id, i.content, i.timestamp, i.group_id, u.username, u.profile_picture
        FROM inserted i
        JOIN users u ON i.user_id = u.id;
        "#,
        user_id,
        content,
        group_id
    )
    .fetch_one(db)
    .await
}

pub async fn fetch_group_type(group_id: i32, db: &PgPool) -> Result<i32, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        SELECT group_type
        FROM groups
        WHERE id = $1
        "#,
        group_id
    )
    .fetch_one(db)
    .await?;

    Ok(result.group_type)
}

pub async fn delete_group(group_id: i32, db: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM groups
        WHERE id = $1
        "#,
        group_id
    )
    .execute(db)
    .await?;
    Ok(())
}

pub async fn fetch_dm_id(user_id: i32, friend_id: i32, db: &PgPool) -> Result<i32, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        SELECT g.id
        FROM groups g
        WHERE g.group_type = 2
        AND EXISTS (
            SELECT FROM group_members gm WHERE gm.group_id = g.id AND gm.user_id = $1
        )
        AND EXISTS (
            SELECT FROM group_members gm WHERE gm.group_id = g.id AND gm.user_id = $2
        )
        "#,
        user_id,
        friend_id
    )
    .fetch_one(db)
    .await?;

    Ok(result.id)
}