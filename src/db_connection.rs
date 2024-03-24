
use sqlx::postgres::PgPoolOptions;
 pub async fn connect() -> sqlx::PgPool {
 PgPoolOptions::new()
.max_connections(20)
.connect("postgres://postgres:manakamana@localhost/rust-api").await.unwrap()
}