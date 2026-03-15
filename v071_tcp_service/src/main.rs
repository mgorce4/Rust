

use clap::Parser;
use v070_cli_services::configuration::Configuration;
use v070_cli_services::app_builder::run_app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Configuration::parse();
    run_app(config).await
}
