use crate::database::{CreateUser, User};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use std::sync::Arc;
use tokio_postgres::Client;

pub async fn index() -> &'static str {
    "Rest full Api made with Rust"
}

pub async fn view_user(
    Path(id): Path<i32>,
    State(db_state): State<Arc<Client>>,
) -> Result<(StatusCode, Json<Vec<User>>), StatusCode> {
    let query = "SELECT * FROM users WHERE id = $1";

    let rows = db_state.query(query, &[&id]).await;

    // TODO: Quando não encotra usuario continua retornado OK, o certo seria NOT_FOUND
    match rows {
        Ok(result) => {
            let user: Vec<User> = result.into_iter().map(|row| User::from(row)).collect();
            return Ok((StatusCode::OK, Json(user)));
        }
        Err(_) => return Err(StatusCode::NOT_FOUND),
    };
}

pub async fn update_user(
    Path(id): Path<i32>,
    State(db_state): State<Arc<Client>>,
    Json(payload): Json<CreateUser>,
) -> StatusCode {
    // "UPDATE users SET name = $1, email = $2 WHERE id = $3"
    let query = "UPDATE users SET name = $1, email = $2 WHERE id = $3";

    match db_state
        .query(query, &[&payload.name, &payload.email, &id])
        .await
    {
        Ok(_) => return StatusCode::OK,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };
}

pub async fn delete_user(Path(id): Path<i32>, State(db_state): State<Arc<Client>>) -> StatusCode {
    let query = "DELETE FROM users WHERE id = $1";

    match db_state.query(query, &[&id]).await {
        Ok(_) => return StatusCode::OK,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };
}

pub async fn view_all_users(
    State(db_state): State<Arc<Client>>,
) -> Result<(StatusCode, Json<Vec<User>>), StatusCode> {
    let query = "SELECT id, name, email FROM users";

    let rows = db_state.query(query, &[]).await;

    match rows {
        Ok(result) => {
            let users: Vec<User> = result.into_iter().map(|row| User::from(row)).collect();
            return Ok((StatusCode::OK, Json(users)));
        }
        Err(_) => return Err(StatusCode::NOT_FOUND),
    }
}

pub async fn create_user(
    State(db_state): State<Arc<Client>>,
    Json(payload): Json<CreateUser>,
) -> StatusCode {
    let query = "INSERT INTO users (name, email) VALUES ($1, $2)";

    match db_state
        .query(query, &[&payload.name, &payload.email])
        .await
    {
        Ok(_) => return StatusCode::CREATED,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };
}
