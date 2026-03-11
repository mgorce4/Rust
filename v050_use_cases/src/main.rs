

use clap::Parser;
use v050_use_cases::configuration::Configuration;
use v050_use_cases::app_builder::run_app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Configuration::parse();
    run_app(config).await
}
