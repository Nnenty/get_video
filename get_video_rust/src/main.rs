use std::{env, io};

use reqwest;
use tokio;
use tracing::debug;
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::new("debug"))
        .init();

    tokio::task::spawn_blocking(|| {
        let filename = "video.mp4";
        let mut file = match std::fs::File::create(filename) {
            Ok(file) => {
                debug!("Create file with name `{filename}`");

                file
            }
            Err(err) => match err.kind() {
                io::ErrorKind::AlreadyExists => std::fs::File::open(filename).unwrap(),
                _ => panic!("error to create a new file from response: {err}"),
            },
        };

        // only get video from server
        let port = env::var("PORT").expect("specify `PORT` env");
        let addr = format!("http://127.0.0.1:{}/video", port);

        reqwest::blocking::get(addr)
            .unwrap()
            .copy_to(&mut file)
            .unwrap();
    })
    .await
    .unwrap();

    debug!("Response received");
}
