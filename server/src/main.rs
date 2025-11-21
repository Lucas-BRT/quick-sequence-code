use axum::Router;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

const PORT: u16 = 1919;
static WASM_FILES_PATH: &str = "pkg";
static STATIC_FILES_PATH: &str = "public";

#[tokio::main]
async fn main() {
    let app = Router::new().fallback_service(
        ServeDir::new(WASM_FILES_PATH).fallback(ServeDir::new(STATIC_FILES_PATH)),
    );

    let listener = TcpListener::bind(format!("0.0.0.0:{}", PORT))
        .await
        .expect("failed to bind listener");

    println!("Server started on http://localhost:{}", PORT);

    axum::serve(listener, app)
        .await
        .expect("failed to start server");
}
