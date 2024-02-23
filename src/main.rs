use std::sync::Arc;

use handler::Handler;
use repositories::{authors::AuthorRepository, SqlxRepository};
use sqlx::postgres::PgPoolOptions;
mod domain;
mod handler;
mod repositories;
mod routes;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let conn_str = std::env::var("DATABASE_URL").expect("Could not fetch connection string.");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&conn_str)
        .await
        .expect("Couldn't connect to the database");
    let repository = SqlxRepository::new(pool);
    let handler = Handler::new(Arc::new(repository));

    let app = routes::configure_routes().with_state(handler);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
