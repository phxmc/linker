mod link;

use axum::extract::Path;
use tokio::net::TcpListener;
use axum::Router;
use axum::body::Body;
use axum::http::header::LOCATION;
use axum::routing::get;
use axum::http::StatusCode;
use axum::response::{Response, IntoResponse};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/{id}", get(link));

    let addr = "0.0.0.0:8049";

    println!("running on {addr}");
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn link(Path(path): Path<String>) -> Response {
    let id = path.strip_suffix("/").unwrap_or(&path);
    let link = link::parser::parse(id.to_string());

    match link {
        Ok(link) => Response::builder()
            .status(StatusCode::PERMANENT_REDIRECT)
            .header(LOCATION, link.config.to)
            .body(Body::empty())
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap(),
    }
}
