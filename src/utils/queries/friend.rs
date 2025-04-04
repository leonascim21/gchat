use sqlx::PgPool;
use crate::utils::types::{Friend, FriendRequest};

pub async fn fetch_friends_for_user(user_id: i32, db: &PgPool ) -> Result<Vec<Friend>, sqlx::Error> {
    sqlx::query_as!(
        Friend,
        r#"
        SELECT f.friend_id AS id, u.username, u.profile_picture
        FROM friendships f 
        JOIN users u ON f.friend_id = u.id 
        WHERE f.user_id = $1
        "#,
        user_id
    )
    .fetch_all(db).await
}

pub async fn fetch_outgoing_requests(user_id: i32, db: &PgPool) -> Result<Vec<FriendRequest>, sqlx::Error> {
    sqlx::query_as!(
        FriendRequest,
        r#"
        SELECT fr.sender_id, fr.receiver_id, u.username
        FROM friend_requests fr
        JOIN users u ON fr.receiver_id = u.id
        WHERE fr.sender_id = $1
        "#,
        user_id
    )
    .fetch_all(db).await
}

pub async fn fetch_incoming_requests(user_id: i32, db: &PgPool) -> Result<Vec<FriendRequest>, sqlx::Error> {
    sqlx::query_as!(
        FriendRequest,
        r#"
        SELECT fr.sender_id, fr.receiver_id, u.username
        FROM friend_requests fr
        JOIN users u ON fr.sender_id = u.id
        WHERE fr.receiver_id = $1
        "#,
        user_id
    )
    .fetch_all(db).await
}

pub async fn create_friend_request(sender_id: i32, receiver_id: i32, db: &PgPool) 
-> Result<FriendRequest, sqlx::Error> {
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
        sender_id,
        receiver_id,
    ).fetch_one(db).await?;

    Ok(FriendRequest{
        sender_id: result.sender_id, 
        receiver_id: result.receiver_id, 
        username: result.username
    })
}

pub async fn create_friendship(user_id: i32, friend_id: i32, db: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO friendships (user_id, friend_id)
        VALUES ($1, $2);
        "#,
        user_id,
        friend_id
    ).execute(db).await?;

    Ok(())
}

pub async fn delete_friend_request(sender_id: i32, receiver_id: i32, db: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE
        FROM friend_requests 
        WHERE sender_id = $1 AND receiver_id = $2
        "#,
        sender_id,
        receiver_id
    ).execute(db).await?;

    Ok(())
}

pub async fn delete_friendship(user_id: i32, friend_id: i32, db: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE
        FROM friendships 
        WHERE user_id = $1 AND friend_id = $2
        "#, 
        user_id,
        friend_id
    ).execute(db).await?;

    Ok(())
}

pub async fn fetch_friend_request(sender_id: i32, receiver_id: i32, db: &PgPool) 
-> Result<FriendRequest, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        SELECT * 
        FROM friend_requests 
        WHERE sender_id = $1 AND receiver_id = $2
        "#,
        sender_id,
        receiver_id
    ).fetch_one(db).await?;

    Ok(FriendRequest{
        sender_id: result.sender_id, 
        receiver_id: result.receiver_id, 
        username: "".to_string()
    })
}