use axum::{ http, extract };
use serde::{ Deserialize, Serialize };
use sqlx::{ Pool, Sqlite, FromRow, query, query_as };
use chrono::{ DateTime, Utc };
use http::StatusCode;

pub type SqlitePool = Pool<Sqlite>;

#[derive(Debug, Serialize, FromRow)]
pub struct Section {
    name: String,
    aname: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Section {
    fn new(name: String, aname: String) -> Self {
        Self {
            name,
            aname,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Debug, Serialize, FromRow)]
pub struct SectionResponse {
    id: i32,
    name: String,
    aname: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl SectionResponse {
    fn new(
        id: i32,
        name: String,
        aname: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>
    ) -> Self {
        Self {
            id: id,
            name: name,
            aname: aname,
            created_at: created_at,
            updated_at: updated_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Create {
    name: String,
    aname: String,
}

pub async fn create(
    extract::State(pool): extract::State<SqlitePool>,
    axum::Json(payload): axum::Json<Create>
) -> Result<(StatusCode, axum::Json<Section>), StatusCode> {
    let section = Section::new(payload.name, payload.aname);
    let res = query(
        r#"INSERT INTO Sections (name,aname,created_at,updated_at) VALUES ($1,$2,$3,$4)"#
    )
        .bind(&section.name)
        .bind(&section.aname)
        .bind(&section.created_at)
        .bind(&section.updated_at)
        .execute(&pool).await;

    match res {
        Ok(_) => Ok((StatusCode::OK, axum::Json(section))),
        Err(e) => {
            println!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn read(extract::State(pool): extract::State<SqlitePool>) -> Result<
    axum::Json<Vec<SectionResponse>>,
    StatusCode> {
    let res = query_as::<_, SectionResponse>(r#"SELECT * FROM Sections"#).fetch_all(&pool).await;

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
) -> Result<(StatusCode, axum::Json<Section>), StatusCode> {
    let now = Utc::now();
    let section = Section::new(payload.name, payload.aname);
    let res = query(r#"UPDATE Sections SET name=$1,aname=$2,updated_at=$3 WHERE id=$4"#)
        .bind(&section.name)
        .bind(&section.aname)
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
        Ok(_) => Ok((StatusCode::OK, axum::Json(section))),
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
    let res = query(r#"DELETE FROM Sections WHERE id=$1"#)
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
