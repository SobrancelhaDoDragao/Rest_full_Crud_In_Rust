///! Rest full crud in Rust
use axum::Router;
use std::sync::Arc;

mod database;
mod handlers;
mod routes;

use database::set_database;
use routes::all_routes;

#[tokio::main]
async fn main() {
    let db_pool = match set_database().await {
        Ok(db) => {
            println!("Conectado ao banco de dados com sucesso!");
            db
        }
        Err(e) => {
            panic!("Erro ao conectar ao banco de dados {:?}", e);
        }
    };

    let app = Router::new()
        .merge(all_routes())
        .with_state(Arc::new(db_pool));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Servidor url: 0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}
