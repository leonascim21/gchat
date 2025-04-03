use sqlx::PgPool;
pub struct Group {
    pub id: i32,
    pub name: String,
    pub profile_picture: Option<String>,
}

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