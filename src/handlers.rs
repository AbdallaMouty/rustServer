use axum::{ http, extract };
use serde::{ Deserialize, Serialize };
use sqlx::{ Pool, Sqlite, FromRow, query, query_as };
use chrono::{ DateTime, Utc };
use http::StatusCode;
pub type SqlitePool = Pool<Sqlite>;

#[derive(Debug, Serialize, FromRow)]
pub struct Quote {
    book: String,
    quote: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Quote {
    fn new(book: String, quote: String) -> Self {
        let now = Utc::now();
        Self {
            book,
            quote,
            created_at: now,
            updated_at: now,
        }
    }
}
#[derive(Debug, Serialize, FromRow)]
pub struct ReadQuote {
    id: i32,
    book: String,
    quote: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ReadQuote {
    fn new(
        id: i32,
        book: String,
        quote: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>
    ) -> Self {
        Self {
            id: id,
            book: book,
            quote: quote,
            created_at: created_at,
            updated_at: updated_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Create {
    book: String,
    quote: String,
}

pub async fn create(
    extract::State(pool): extract::State<SqlitePool>,
    axum::Json(payload): axum::Json<Create>
) -> Result<(StatusCode, axum::Json<Quote>), StatusCode> {
    let quote = Quote::new(payload.book, payload.quote);
    let res = query(
        r#"INSERT INTO quotes (book, quote, created_at, updated_at)
    VALUES ($1, $2, $3, $4)"#
    )
        .bind(&quote.book)
        .bind(&quote.quote)
        .bind(&quote.created_at)
        .bind(&quote.updated_at)
        .execute(&pool).await;

    match res {
        Ok(_) => Ok((StatusCode::OK, axum::Json(quote))),
        Err(e) => {
            println!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn read(extract::State(pool): extract::State<SqlitePool>) -> Result<
    axum::Json<Vec<ReadQuote>>,
    StatusCode
> {
    let res = query_as::<_, ReadQuote>(r#"SELECT * FROM quotes"#).fetch_all(&pool).await;
    match res {
        Ok(quotes) => Ok(axum::Json(quotes)),
        Err(e) => {
            println!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update(
    extract::State(pool): extract::State<SqlitePool>,
    extract::Path(id): extract::Path<i32>,
    axum::Json(payload): axum::Json<Create>
) -> StatusCode {
    let now = Utc::now();

    let res = query(r#"UPDATE quotes SET book = $1, quote = $2, updated_at = $3 WHERE id = $4"#)
        .bind(&payload.book)
        .bind(&payload.quote)
        .bind(now)
        .bind(id)
        .execute(&pool).await
        .map(|res| {
            match res.rows_affected() {
                0 => StatusCode::NOT_FOUND,
                _ => StatusCode::OK,
            }
        });

    match res {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            println!("{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn delete(
    extract::State(pool): extract::State<SqlitePool>,
    extract::Path(id): extract::Path<i32>
) -> StatusCode {
    let res = query(r#"DELETE FROM quotes WHERE id = $1"#)
        .bind(id)
        .execute(&pool).await
        .map(|res| {
            match res.rows_affected() {
                0 => StatusCode::NOT_FOUND,
                _ => StatusCode::OK,
            }
        });

    match res {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            println!("{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
