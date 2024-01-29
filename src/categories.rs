use axum::{ http, extract };
use serde::{ Deserialize, Serialize };
use sqlx::{ Pool, Sqlite, FromRow, query, query_as };
use chrono::{ DateTime, Utc };
use http::StatusCode;

pub type SqlitePool = Pool<Sqlite>;

#[derive(Debug, Serialize, FromRow)]
pub struct Category {
    secId: i32,
    name: String,
    aname: String,
    IMG: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Category {
    fn new(secId: i32, name: String, aname: String, IMG: String) -> Self {
        Self {
            secId,
            name,
            aname,
            IMG,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Debug, Serialize, FromRow)]
pub struct CatResponse {
    id: i32,
    name: String,
    aname: String,
    secId: i32,
    IMG: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl CatResponse {
    fn new(
        id: i32,
        name: String,
        aname: String,
        secId: i32,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        IMG: String
    ) -> Self {
        Self {
            id: id,
            name: name,
            aname: aname,
            secId: secId,
            IMG: IMG,
            created_at: created_at,
            updated_at: updated_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Create {
    name: String,
    aname: String,
    secId: i32,
    IMG: String,
}

pub async fn create(
    extract::State(pool): extract::State<SqlitePool>,
    axum::Json(payload): axum::Json<Create>
) -> Result<(StatusCode, axum::Json<Category>), StatusCode> {
    let category = Category::new(payload.secId, payload.name, payload.aname, payload.IMG);
    let res = query(
        r#"INSERT INTO Categories (name,aname,secId,IMG,created_at,updated_at) VALUES ($1,$2,$3,$4,$5,$6)"#
    )
        .bind(&category.name)
        .bind(&category.aname)
        .bind(&category.secId)
        .bind(&category.IMG)
        .bind(&category.created_at)
        .bind(&category.updated_at)
        .execute(&pool).await;

    match res {
        Ok(_) => Ok((StatusCode::OK, axum::Json(category))),
        Err(e) => {
            println!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn read(extract::State(pool): extract::State<SqlitePool>) -> Result<
    axum::Json<Vec<CatResponse>>,
    StatusCode> {
    let res = query_as::<_, CatResponse>(r#"SELECT * FROM Categories"#).fetch_all(&pool).await;

    match res {
        Ok(categories) => Ok(axum::Json(categories)),
        Err(e) => {
            println!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn read_by_id(
    extract::State(pool): extract::State<SqlitePool>,
    extract::Path(secId): extract::Path<i32>
) -> Result<axum::Json<Vec<CatResponse>>, StatusCode> {
    let res = query_as::<_, CatResponse>(r#"SELECT * FROM Categories WHERE secId=$1"#)
        .bind(secId)
        .fetch_all(&pool).await;

    match res {
        Ok(sections) => Ok(axum::Json(sections)),
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
) -> Result<(StatusCode, axum::Json<Category>), StatusCode> {
    let now = Utc::now();
    let category = Category::new(payload.secId, payload.name, payload.aname, payload.IMG);
    let res = query(r#"UPDATE Categories SET name=$1,aname=$2,IMG=$4,updated_at=$5 WHERE id=$6"#)
        .bind(&category.name)
        .bind(&category.aname)
        .bind(&category.IMG)
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
        Ok(_) => Ok((StatusCode::OK, axum::Json(category))),
        Err(e) => {
            println!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete(
    extract::State(pool): extract::State<SqlitePool>,
    extract::Path(id): extract::Path<i32>
) -> Result<StatusCode, StatusCode> {
    let res = query(r#"DELETE FROM Categories WHERE id=$1"#)
        .bind(id)
        .execute(&pool).await
        .map(|res| {
            match res.rows_affected() {
                0 => StatusCode::NOT_FOUND,
                _ => StatusCode::OK,
            }
        });

    match res {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => {
            println!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
