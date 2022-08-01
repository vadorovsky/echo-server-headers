use std::net::SocketAddr;

use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::HeaderMap,
    response::Response,
    routing::get,
    Router,
};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(root));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root(headers: Headers) -> String {
    let mut body = String::new();
    for (key, value) in headers.map.iter() {
        body.push_str(format!("{}: {:?}\n", key, value).as_str());
    }
    body
}

struct Headers {
    map: HeaderMap,
}

#[async_trait]
impl<B> FromRequest<B> for Headers
where
    B: Send,
{
    type Rejection = Response;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let headers = req.headers().to_owned();
        Ok(Headers { map: headers })
    }
}
