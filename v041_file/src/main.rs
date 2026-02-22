

use clap::Parser;
use v041_file::configuration::Configuration;
use v041_file::app_builder::run_app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Configuration::parse();
    run_app(config).await
}
