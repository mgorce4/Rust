

use clap::Parser;
use v030_domain::configuration::Configuration;
use v030_domain::app_builder::run_app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Configuration::parse();
    run_app(config).await
}
