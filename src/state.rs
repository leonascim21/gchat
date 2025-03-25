use sqlx::PgPool;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct ServerState {
    pub db: PgPool,
    pub tx: broadcast::Sender<String>,
}