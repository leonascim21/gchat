use sqlx::PgPool;
use axum::extract::ws::Message;
use std::{
    collections::HashMap,
    sync::Arc,
};
use tokio::sync::{mpsc, Mutex};
use uuid::Uuid;

pub type ChannelMap = HashMap<i32, HashMap<Uuid, mpsc::UnboundedSender<Message>>>;

#[derive(Clone)]
pub struct ServerState {
    pub db: PgPool,
    pub channels: Arc<Mutex<ChannelMap>>
}