use std::net::SocketAddr;

use clap::Parser;
use hudsucker::Proxy;
use tokio::signal;

use self::config::GhostConfig;
use self::handler::GhostHandler;

mod action;
mod authority;
mod client;
mod config;
mod handler;
mod http;
mod keywords;
mod logging;
mod modifier;

async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
}

#[tokio::main]
async fn main() {
    let config = GhostConfig::parse();
    logging::init();

    let ca = authority::load(&config).await;
    let proxy = Proxy::builder()
        .with_addr(SocketAddr::from(([0, 0, 0, 0], config.port)))
        .with_client(client::build())
        .with_ca(ca)
        .with_http_handler(GhostHandler::new(&config))
        .build();

    if let Err(e) = proxy.start(shutdown_signal()).await {
        log::error!("Proxy error: {:?}", e);
    }
}
