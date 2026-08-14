//! Local preview server for the prerendered `dist/` output.
//!
//! Serves the static site exactly as a production static host would,
//! including a real HTTP 404 status for unmatched paths (via `dist/404.html`).

use axum::http::StatusCode;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::set_status::SetStatus;

#[tokio::main]
async fn main() {
    let not_found = SetStatus::new(ServeFile::new("dist/404.html"), StatusCode::NOT_FOUND);
    let serve_dir = ServeDir::new("dist").not_found_service(not_found);

    let app = axum::Router::new().fallback_service(serve_dir);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
        .await
        .expect("failed to bind 127.0.0.1:4000");

    println!("Serving dist/ at http://127.0.0.1:4000");

    axum::serve(listener, app).await.expect("server error");
}
