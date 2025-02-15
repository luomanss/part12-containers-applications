use axum::{
    extract::{Path, State},
    http::{Method, StatusCode},
    routing::{delete, get, patch, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool, Row};
use tower_http::cors::{Any, CorsLayer};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers(Any);

    let app = Router::new()
        .route("/persons", get(persons))
        .route("/persons", post(create_person))
        .route("/persons/{id}", get(person))
        .route("/persons/{id}", patch(update_person))
        .route("/persons/{id}", delete(delete_person))
        .layer(cors)
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn persons(State(pool): State<PgPool>) -> (StatusCode, Json<Option<Vec<Person>>>) {
    sqlx::query("SELECT * FROM person")
        .map(|row: sqlx::postgres::PgRow| Person {
            id: row.get(0),
            name: row.get(1),
            number: row.get(2),
        })
        .fetch_all(&pool)
        .await
        .map(|persons| (StatusCode::OK, Json(Some(persons))))
        .unwrap_or_else(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(None)))
}

async fn create_person(
    State(pool): State<PgPool>,
    Json(person): Json<CreatePerson>,
) -> (StatusCode, Json<Option<Person>>) {
    let row = sqlx::query("INSERT INTO person (name, number) VALUES ($1, $2) RETURNING *")
        .bind(person.name)
        .bind(person.number)
        .map(|row: sqlx::postgres::PgRow| Person {
            id: row.get(0),
            name: row.get(1),
            number: row.get(2),
        })
        .fetch_one(&pool)
        .await;

    match row {
        Ok(person) => (StatusCode::CREATED, Json(Some(person))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}

async fn person(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> (StatusCode, Json<Option<Person>>) {
    let row = sqlx::query("SELECT * FROM person WHERE id = $1")
        .bind(id)
        .map(|row: sqlx::postgres::PgRow| Person {
            id: row.get(0),
            name: row.get(1),
            number: row.get(2),
        })
        .fetch_one(&pool)
        .await;

    match row {
        Ok(person) => (StatusCode::OK, Json(Some(person))),
        Err(_) => (StatusCode::NOT_FOUND, Json(None)),
    }
}

async fn update_person(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(person): Json<CreatePerson>,
) -> (StatusCode, Json<Option<Person>>) {
    let row = sqlx::query("UPDATE person SET name = $1, number = $2 WHERE id = $3 RETURNING *")
        .bind(person.name)
        .bind(person.number)
        .bind(id)
        .map(|row: sqlx::postgres::PgRow| Person {
            id: row.get(0),
            name: row.get(1),
            number: row.get(2),
        })
        .fetch_one(&pool)
        .await;

    match row {
        Ok(person) => (StatusCode::OK, Json(Some(person))),
        Err(_) => (StatusCode::NOT_FOUND, Json(None)),
    }
}

async fn delete_person(State(pool): State<PgPool>, Path(id): Path<i32>) -> (StatusCode, Json<()>) {
    let row = sqlx::query("DELETE FROM person WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await;

    match row {
        Ok(_) => (StatusCode::NO_CONTENT, Json(())),
        Err(_) => (StatusCode::NOT_FOUND, Json(())),
    }
}

#[derive(Deserialize)]
struct CreatePerson {
    name: String,
    number: String,
}

#[derive(Serialize)]
struct Person {
    id: i32,
    name: String,
    number: String,
}
