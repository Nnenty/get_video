use axum::{
    self,
    body::Body,
    http::{header, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use std::env;
use tokio::{self, io::AsyncReadExt};
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
        .route("/", get(usage))
        .route("/video", get(video));

    axum::serve(listener, app).await.unwrap();
}
async fn usage() -> axum::response::Html<String> {
    let mut usage_html = tokio::fs::File::open("usage.html")
        .await
        .expect("usage.html is not found");
    let mut content = String::new();
    usage_html.read_to_string(&mut content).await.unwrap();

    // Html() parse it into:
    //
    // Response::builder()
    //     .status(StatusCode::OK)
    //     .header("Content-Type", "text/html")
    //     .body(Body::from(content))
    //     .unwrap()
    axum::response::Html(content)
}
async fn video() -> impl IntoResponse {
    debug!("Client connected");

    let filename = "shikonoko.mp4";
    let file = match tokio::fs::File::open(filename).await {
        Ok(file) => file,
        Err(err) => match err.kind() {
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

    debug!("Client got {} bytes", filemeta.len());
    Ok((StatusCode::OK, headers, body))
}
