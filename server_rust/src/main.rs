use axum::{
    self,
    body::Body,
    http::{header, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use std::env;
use tokio;
use tokio_util::io::ReaderStream;
use tracing::debug;
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::new("debug"))
        .init();

    let port = env::var("PORT").expect("specify `PORT` env");
    let addr = format!("127.0.0.1:{}", port);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    debug!("Listen on {addr}");

    let app = Router::new()
        .route(
            "/",
            get(|| async {
                debug!("Client connect");
                StatusCode::OK
            }),
        )
        .route("/video", get(handler_video));

    axum::serve(listener, app).await.unwrap();
}
async fn handler_video() -> impl IntoResponse {
    debug!("Client connect");

    let filename = "shikonoko.mp4";
    let file = match tokio::fs::File::open(filename).await {
        Ok(shikonoko) => shikonoko,
        Err(e) => match e.kind() {
            tokio::io::ErrorKind::NotFound => {
                return Err((StatusCode::NOT_FOUND, "Could not find file".to_owned()));
            }
            _ => return Err((StatusCode::BAD_REQUEST, "Bad request".to_owned())),
        },
    };

    let filemeta = match file.metadata().await {
        Ok(meta) => meta,
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Server error: {err}"),
            ));
        }
    };

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let headers = [
        (header::CONTENT_TYPE, "video/mp4;".to_owned()),
        (header::ACCEPT_RANGES, "bytes".to_owned()),
        (header::CONTENT_LENGTH, filemeta.len().to_string()),
    ];

    debug!("Client could got {} bytes", filemeta.len());
    Ok((StatusCode::OK, headers, body))
}
