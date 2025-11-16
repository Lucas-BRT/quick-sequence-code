use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::get,
};
use std::path::PathBuf;
use tokio::{fs::read_to_string, net::TcpListener};
use tower_http::services::ServeDir;

const PORT: u16 = 1919;
static WASM_FILES_PATH: &str = "pkg";

async fn index() -> impl IntoResponse {
    let path = PathBuf::from("public/index.html");
    Html(read_to_string(path).await.expect("failed to read file"))
}

#[tokio::main]
async fn main() {
    let static_files = ServeDir::new(WASM_FILES_PATH);
    let app = Router::new()
        .route("/", get(index))
        .fallback_service(static_files);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", PORT))
        .await
        .expect("failed to bind listener");

    println!("Server started on http://localhost:{}", PORT);

    axum::serve(listener, app)
        .await
        .expect("failed to start server");
}
