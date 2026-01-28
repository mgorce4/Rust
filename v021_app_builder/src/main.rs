
pub mod configuration;


use clap::Parser;
use configuration::Configuration;
mod app_builder;
use app_builder::run_app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Configuration::parse();
    run_app(config).await
}
