use serde::Serialize;
use sqlx::PgPool;


#[derive(Serialize)]
struct UserStats {
    messages_sent: i64,
    favorite_group: String,
    best_friend: String,
    longest_message: String,
}

pub async fn fetch_stats(user_id: i32, db: &PgPool) -> Result<String, sqlx::Error> {
    let messages_sent = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*)
        FROM messages
        WHERE user_id = $1
        "#,
        user_id
    ).fetch_one(db).await?;

    let favorite_group = sqlx::query_scalar!(
        r#"
        SELECT g.name
        FROM groups g
        JOIN messages m ON g.id = m.group_id
        WHERE m.user_id = $1 AND g.group_type = 1
        GROUP BY g.name
        ORDER BY COUNT(m.id) DESC LIMIT 1;
        "#,
        user_id
    ).fetch_all(db).await?;

    let best_friend = sqlx::query_scalar!(
        r#"
        SELECT u.username
        FROM group_members gm
        JOIN users u ON gm.user_id = u.id
        WHERE gm.group_id = (
            SELECT g.id
            FROM groups g
            JOIN messages m ON g.id = m.group_id
            WHERE g.group_type = 2 
            AND m.user_id = $1
            GROUP BY g.id
            ORDER BY COUNT(m.id) DESC
            LIMIT 1
        )
        AND gm.user_id != $1;
        "#,
        user_id
    ).fetch_all(db).await?;

    let longest_message = sqlx::query_scalar!(
        r#"
        SELECT m.content
        FROM messages m
        JOIN groups g ON m.group_id = g.id
        WHERE m.user_id = $1 AND g.group_type != 3
        ORDER BY LENGTH(m.content) DESC
        LIMIT 1;
        "#,
        user_id
    ).fetch_all(db).await?;

    let messages_sent_val = messages_sent.unwrap();
    let favorite_group_val = favorite_group.iter().next().unwrap();
    let best_friend_val = best_friend.iter().next().unwrap();
    let longest_message_val = longest_message.iter().next().unwrap();

    let stats = UserStats {
        messages_sent: messages_sent_val,
        favorite_group: favorite_group_val.to_string(),
        best_friend: best_friend_val.to_string(),
        longest_message: longest_message_val.to_string(),
    };

    let json_output = serde_json::to_string_pretty(&stats).map_err(|_| sqlx::Error::RowNotFound);
    
    return json_output;
}