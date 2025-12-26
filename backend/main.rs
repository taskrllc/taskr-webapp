mod app;
mod db;
mod util;

use axum::Router;
use std::{env, path::PathBuf};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let address = env::var("ADDRESS").unwrap_or("localhost:3000".to_string());

    let static_dir: PathBuf = env::current_dir()
        .expect("failed to get working directory")
        .join("static");

    let app = Router::new().nest_service("/", ServeDir::new(static_dir.to_str().unwrap()));

    let listener = TcpListener::bind(&address).await.expect("tcp bind error");

    println!("Serving at http://{}", address);

    axum::serve(listener, app)
        .await
        .expect("internal server error");
}
