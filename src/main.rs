mod link;
mod reference;

use tokio::net::TcpListener;
use axum::extract::{Path, Query};
use axum::Router;
use axum::body::Body;
use axum::http::header::LOCATION;
use axum::routing::get;
use axum::http::StatusCode;
use axum::response::{Response, IntoResponse};
use serde::Deserialize;
use crate::reference::entity::Reference;
use crate::reference::storage;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/{id}", get(link));

    let addr = "0.0.0.0:8049";

    println!("running on {addr}");
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Deserialize)]
struct Params {
    #[serde(default, rename="ref")]
    reference: String,
}

async fn link(Path(path): Path<String>, Query(params): Query<Params>) -> Response {
    let id = path.strip_suffix("/").unwrap_or(&path);
    let link = link::parser::parse(id.to_string());

    if params.reference == "" {
        println!("/{path}")
    } else {
        println!("/{path}?ref={}", params.reference);
    }

    let reference = storage::get(params.reference.clone());

    match reference {
        Ok(reference) => {
            let mut new_reference = Reference {
                metrics: reference.metrics
            };

            new_reference.metrics.uses += 1;

            storage::set(params.reference, new_reference).unwrap();
        },
        Err(_) => {}
    }

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
