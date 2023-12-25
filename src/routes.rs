use crate::handlers;
use axum::{
    routing::{get, post},
    Router,
};

use std::sync::Arc;

// Users routes:
// - `GET /`: index page.
// - `GET /users/`: return a JSON list of Users.
// - `POST /users/`: create a new User.
// - `PATCH /users/:id`: update a specific User.
// - `DELETE /users/:id`: delete a specific User.
pub fn all_routes() -> Router<Arc<tokio_postgres::Client>> {
    Router::new()
        .route("/", get(handlers::index))
        .route(
            "/users/",
            post(handlers::create_user).get(handlers::view_all_users),
        )
        .route(
            "/users/:id",
            get(handlers::view_user)
                .patch(handlers::update_user)
                .delete(handlers::delete_user),
        )
}
