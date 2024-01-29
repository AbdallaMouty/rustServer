use axum::{ http::StatusCode, extract };
use serde::{ Deserialize, Serialize };
use sqlx::{ Pool, Sqlite, FromRow, query, query_as };
use chrono::{ Date, DateTime, Utc };

pub type SqlitePool = Pool<Sqlite>;

#[derive(Debug, Serialize, FromRow)]
pub struct Item {
    catId: i32,
    name: String,
    aname: String,
    IMG: String,
    price: String,
    desc: String,
    adesc: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
impl Item {
    fn new(
        catId: i32,
        name: String,
        aname: String,
        IMG: String,
        price: String,
        desc: String,
        adesc: String
    ) -> Self {
        Self {
            catId,
            name,
            aname,
            IMG,
            price,
            desc,
            adesc,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
#[derive(Debug, Serialize, FromRow)]
pub struct ItemRes {
    catId: i32,
    id: i32,
    name: String,
    aname: String,
    IMG: String,
    price: String,
    desc: String,
    adesc: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
impl ItemRes {
    fn new(
        catId: i32,
        id: i32,
        name: String,
        aname: String,
        IMG: String,
        price: String,
        desc: String,
        adesc: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>
    ) -> Self {
        Self {
            catId,
            id,
            name,
            aname,
            IMG,
            price,
            desc,
            adesc,
            created_at,
            updated_at,
        }
    }
}
#[derive(Deserialize)]
pub struct Create {
    catId: i32,
    name: String,
    aname: String,
    IMG: String,
    price: String,
    desc: String,
    adesc: String,
}

pub async fn create(
    extract::State(pool): extract::State<SqlitePool>,
    axum::Json(payload): axum::Json<Create>
) -> Result<(StatusCode, axum::Json<Item>), StatusCode> {
    let item = Item::new(
        payload.catId,
        payload.name,
        payload.aname,
        payload.IMG,
        payload.price,
        payload.desc,
        payload.adesc
    );
    let res = query(
        r#"INSERT INTO Items (catId,name,aname,IMG,price,desc,adesc,created_at,updated_at) 
    VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)"#
    )
        .bind(&item.catId)
        .bind(&item.name)
        .bind(&item.aname)
        .bind(&item.IMG)
        .bind(&item.price)
        .bind(&item.desc)
        .bind(&item.adesc)
        .bind(&item.created_at)
        .bind(&item.updated_at)
        .execute(&pool).await;

    match res {
        Ok(_) => Ok((StatusCode::OK, axum::Json(item))),
        Err(e) => {
            println!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn read(extract::State(pool): extract::State<SqlitePool>) -> Result<
    axum::Json<Vec<ItemRes>>,
    StatusCode> {
    let res = query_as::<_, ItemRes>(r#"SELECT * FROM Items"#).fetch_all(&pool).await;

    match res {
        Ok(items) => Ok(axum::Json(items)),
        Err(e) => {
            println!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn read_by_id(
    extract::State(pool): extract::State<SqlitePool>,
    extract::Path(catId): extract::Path<i32>
) -> Result<axum::Json<Vec<ItemRes>>, StatusCode> {
    let res = query_as::<_, ItemRes>(r#"SELECT * FROM Items WHERE catId=$1"#)
        .bind(catId)
        .fetch_all(&pool).await;

    match res {
        Ok(items) => Ok(axum::Json(items)),
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
) -> Result<(StatusCode, axum::Json<Item>), StatusCode> {
    let now = Utc::now();
    let item = Item::new(
        payload.catId,
        payload.name,
        payload.aname,
        payload.IMG,
        payload.price,
        payload.desc,
        payload.adesc
    );
    let res = query(
        r#"UPDATE Items SET name=$1,aname$2,IMG=$3,price=$4,desc=$5,adesc=$6,updated_at=$7 WHERE id=$8"#
    )
        .bind(&item.name)
        .bind(&item.aname)
        .bind(&item.IMG)
        .bind(&item.price)
        .bind(&item.desc)
        .bind(&item.adesc)
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
        Ok(_) => Ok((StatusCode::OK, axum::Json(item))),
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
    let res = query(r#"DELETE FROM Items WHERE id=$1"#)
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
