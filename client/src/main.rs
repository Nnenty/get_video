use serde::Deserialize;
use std::io;
use toml;

use reqwest;
use tokio;
use tracing::debug;
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

#[derive(Debug, Deserialize)]
struct Server {
    port: i32,
    host: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    server: Server,
}

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

        let toml = "config.toml";
        let toml = std::fs::read_to_string(toml).unwrap();

        let Config {
            server: Server { port, host },
        } = toml::from_str(&toml).unwrap();

        // request video from server
        let addr = format!("http://{host}:{port}/video");

        reqwest::blocking::get(addr)
            .unwrap()
            .copy_to(&mut file)
            .unwrap();
    })
    .await
    .unwrap();

    debug!("Response received");
}
