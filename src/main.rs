mod handlers;
use axum::http;
use axum::routing::{get, post, Router};
use sqlx::{Pool, Sqlite};
use std::env;

pub type SqlitePool = Pool<Sqlite>;

#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>>{
    let port  = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let pool: SqlitePool = Pool::connect("sqlite:///db.db").await?;

    let app = Router::new()
    .route("/", get(handlers::health))
    .route("/quotes",post(handlers::create))
    .with_state(pool);

    axum::Server::bind(&addr.parse().unwrap())
       .serve(app.into_make_service())
       .await
       .unwrap();
    print!("Server running on {}", addr);
    Ok(())
}

