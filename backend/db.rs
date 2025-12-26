use sqlx::PgPool;

#[derive(sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

pub async fn find_user(pool: &PgPool, email: &str) -> Option<User> {
    sqlx::query_as::<_, User>("SELECT id, name, email, password_hash FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten()
}
