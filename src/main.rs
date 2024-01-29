mod handlers;
mod sections;
mod categories;
mod items;
use axum::routing::{ get, post, put, delete, Router };
use sqlx::{ Pool, Sqlite };
use std::env;

pub type SqlitePool = Pool<Sqlite>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let pool: SqlitePool = Pool::connect("sqlite:./db.db").await?;
    let addr: String = format!("0.0.0.0:{}", port);

    macro_rules! router {
        ($module:ident) => {
            Router::new()
                .route("/all",get($module::read))
                .route("/list", get($module::read_by_id))
                .route("/add", post($module::create))
                .route("/edit/:id", put($module::update))
                .route("/delete/:id", delete($module::delete))
        };
    }

    macro_rules! main_router {
        ($module:ident) => {
            Router::new()
            .route("/all",get($module::read))
            .route("/add", post($module::create))
            .route("/edit/:id", put($module::update))
            .route("/delete/:id", delete($module::delete))
        };
    }

    pub async fn health() -> axum::http::StatusCode {
        axum::http::StatusCode::OK
    }

    let app = Router::new()
        .route("/", get(health))
        .nest("/quotes", main_router!(handlers))
        .nest("/sections", main_router!(sections))
        .nest("/categories", router!(categories))
        .nest("/items", router!(items))
        .with_state(pool);

    axum::Server::bind(&addr.parse().unwrap()).serve(app.into_make_service()).await.unwrap();
    Ok(())
}
