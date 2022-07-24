use handlers::services::Handles;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

pub mod gen;
pub mod handlers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let h = handlers::services::build_handler_map().await;

    let shared = Arc::new(h);

    let service = make_service_fn(move |_conn| {
        let sp = shared.clone();
        async {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                let sp = sp.clone();
                r_proxy_service(req, sp)
            }))
        }
    });

    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}

async fn r_proxy_service(
    req: Request<Body>,
    h: Arc<HashMap<String, Box<dyn Handles + Send + Sync>>>,
) -> Result<Response<Body>, hyper::Error> {
    let split: Vec<&str> = req.uri().path().split("/").collect();
    match split[1] {
        "authenticator" => {
            let hh = h.get("authenticator").unwrap();
            hh.handle(req).await
        }

        _ => {
            let mut not_found = Response::new(Body::from("not found"));
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}
