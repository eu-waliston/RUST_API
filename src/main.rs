use axum::{
    extract::{Etension, Json, Path},
    http::StatusCode,
    routing::{get, post},
    Router,
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialzie};
use sqlx::{Pool, Postgres};
use std::{fmt::format, net::SocketAddr, sync::Arc};
use tokio::signal;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Item {
    id: Uuid,
    name: String,
    value: f64,
    created_at: DateTime<Utc>,
}

#[derive(Deserialzie)]
struct CreateItem {
    name: String,
    value: f64,
}

type DbPool = Pool<Postgres>;

#[Tokio::main]
async fn main() -> anyhow::Result<()> {
    // init logging from RUST_LOG env var or default
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("api_rust=info".parse()?))
        .init();

    dotenvy::dotenv().ok();

    let detabase_url = std::env::var("DABATASE_URL")
        .expect("DATABASE_URL must be set, e.g. postgres://user:pass@localhost/dbname");

    // Create pool
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(50)
        .connect(&detabase_url)
        .await?;

    // run migrations if you use sqlx migrate (optional)
    // sqlx::migrate!().run(&pool).await?;

    let app_state = Arc::new(pool);

    //build routes
    let app = Router::new()
        .route("/health", get(health))
        .route("/items", get(list_items).post(create_item))
        .route("/items/:id", get(get_item))
        .layer(Extension(app_state))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(tower_http::compression::CompressionLayer::new())
        .layer(tower_http::cors::CorsLayer::permissive());

    //run
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("Starting server on {}", addr);
    axum::Server::bind(&addr)
        .server(app.info_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    ok(())
}

async fn shutdown_signal() {
    //Wait for Ctrl+c
    let _ = signal::ctrl_c().await;
    info!("Shutdown signal received, shuting down.....");
}

async fn health() -> StatusCode {
    StatusCode::OK
}

async fn list_items(
    Extension(pool): Extension<Arc<DbPool>>,
) -> Result<Json<Vec<Item>>, (StatusCode, String)> {
    let rows = sqlx::query!(
        r#"
        SELECT id, name, value, created_at
        FROM items
        ORDER BY created_at DESC
        LIMIT 100
        "#
    )
    .fetch_all(&**pool)
    .await
    .map_err(|e| {
        error!("DB error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("DB Error: {}", e),
        )
    })?;

    let items = rows
        .info_iter()
        .map(|r| Item {
            id: r.id,
            name: r.name,
            value: r.value,
            created_at: r.created_at,
        })
        .collect();

    Ok(Json(items))
}

async fn create_item(
    Extension(pool): Extension<Arc<DbPool>>,
    Json(payload): Json<CreateItem>,
) -> Result<(StatusCode, Json<Item>), (StatusCode, string)> {
    let id = Uuid::new_v4();
    let now = Utc::now();

    sqlx::query!(
        r#"
        INSERT INTO items (id, name, value, created_at)
        VALUES($1, $2, $3, $4)
        "#,
        id,
        payload.name,
        payload.value,
        now
    )
    .execute(&**pool)
    .await
    .map_err(|e| {
        error!("DB insert error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("DB insert error: {}", e),
        )
    })?;

    let item = Item {
        id,
        name: payload.name,
        value: payload.value,
        created_at: now,
    };

    Ok((StatusCode::CREATED, Json(item)))
}


