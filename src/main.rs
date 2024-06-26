use std::future::Future;
use std::time::Duration;
use axum::{Json, Router};
use axum::http::StatusCode;
use axum::routing::post;
use log::{error, warn};
use log::LevelFilter::Trace;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow};
use tokio::time;

mod old;
mod mymodule;
mod fib;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(Trace)
        .init();
    init_and_connect_to_database("postgres://postgres:postgres@localhost:9000/postgres").await.unwrap();

    let (tx, rx) = std::sync::mpsc::channel::<String>();

    tokio::spawn(async move {
        loop {
            match cleanup_db().await {
                Ok(_) => {}
                Err(sqlx::Error::RowNotFound) => {
                    tx.send("row was not found".into()).unwrap();
                }
                Err(err) => panic!("{:?}", err)
            }
            let _ = time::sleep(Duration::from_secs(10)).await;
        }
    });

    tokio::spawn(async move {
        while let Ok(msg) = rx.recv() {
            warn!("{:?}", msg)
        }
    });

    let router = Router::new().route("/", post(post_message));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:6000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

#[derive(Deserialize)]
struct GetMessageRequest {
    pub id: i32,
}

#[derive(Deserialize)]
struct PostMessageRequest {
    pub msg: String,
}

async fn post_message(Json(post_message_request): Json<PostMessageRequest>) -> Result<Json<Message>, StatusCode> {
    match commit_msg_to_db(post_message_request.msg).await {
        Ok(msg) => Ok(msg.into()),
        Err(err) => {
            error!("{:?}", err);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

#[derive(FromRow)]
#[derive(Serialize)]
struct Message {
    id: i32,
    msg: String,
}

async fn cleanup_db() -> Result<(), sqlx::Error> {
    sqlx::query(r#"
    DELETE FROM messages;
    "#
    ).fetch_one(db()).await.map(|_| ())
}

async fn commit_msg_to_db(msg: String) -> Result<Message, sqlx::Error> {
    sqlx::query_as(r#"
    INSERT INTO messages (msg) VALUES ($1) RETURNING *;
    "#
    ).bind(msg).fetch_one(db()).await
}

async fn get_message(id: i32) -> Result<Message, sqlx::Error> {
    sqlx::query_as(r#"
    SELECT * from messages where id = $1;
    "#
    ).bind(id).fetch_one(db()).await
}

/// Singleton providing the database pool handle
static DB: std::sync::OnceLock<sqlx::PgPool> = std::sync::OnceLock::new();

pub(crate) async fn init_and_connect_to_database(database_url: &str) -> Result<(), sqlx::Error> {
    const MAX_CONNECTIONS: u32 = 4;

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(MAX_CONNECTIONS)
        .connect(database_url)
        .await?;

    sqlx::migrate!()
        .run(&pool)
        .await?;

    DB.set(pool).expect("database handle is stored in static variable");

    Ok(())
}

/// Provides a Postgres connection pool handle.
pub fn db<'a>() -> &'a sqlx::PgPool {
    DB.get().expect("database uninitialized")
}
