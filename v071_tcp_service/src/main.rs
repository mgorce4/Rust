

use clap::Parser;
use v071_tcp_service::configuration::Configuration;
use v071_tcp_service::app_builder::run_app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Configuration::parse();
    run_app(config).await
}
