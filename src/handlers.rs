use axum::http;
use serde::{Deserialize,Serialize};
use axum::{extract};
use sqlx::{Pool, Sqlite};

pub type SqlitePool = Pool<Sqlite>;

#[derive(Debug,Serialize)]
pub struct Quote {
    book : String,
    quote : String,
    created_at : chrono::DateTime<chrono::Utc>,
    updated_at : chrono::DateTime<chrono::Utc>,
}

impl Quote {
    fn new(book: String, quote: String) -> Self {
        let now = chrono::Utc::now();
        Self{
            book,
            quote,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug,Deserialize)]
pub struct Create {
    book : String,
    quote : String,
}

pub async fn health() -> http::StatusCode{
    http::StatusCode::OK
}

pub async fn create (
    extract::State(pool):extract::State<SqlitePool>,
    axum::Json(payload): axum::Json<Create>,
) -> Result<(http::StatusCode,axum::Json<Quote>), http::StatusCode> {
    let quote = Quote::new(payload.book, payload.quote);
    let res = sqlx::query(
        "r#
        INSERT INTO quotes (book, quote, created_at, updated_at)
        VALUES ($1, $2, $3, $4)
        #"
    ).bind(&quote.book)
    .bind(&quote.quote)
    .bind(&quote.created_at)
    .bind(&quote.updated_at)
    .execute(&pool)
    .await;

    match res {
        Ok(_) => Ok((http::StatusCode::OK,axum::Json(quote))),
        Err(e) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}