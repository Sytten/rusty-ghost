use clap::Parser;
use simple_proxy::{Environment, SimpleProxy};

use self::config::GhostConfig;
use self::middleware::GhostMiddleware;

mod action;
mod config;
mod http;
mod keywords;
mod logging;
mod middleware;
mod modifier;

#[tokio::main]
async fn main() {
    let config = GhostConfig::parse();
    logging::init();

    let mut proxy = SimpleProxy::new(config.port, Environment::Production);

    proxy.add_middleware(Box::new(GhostMiddleware::new(&config)));

    let _ = proxy.run().await;
}
