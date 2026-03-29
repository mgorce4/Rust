

use clap::Parser;
use v100_hxml::app_builder::run_app;
use v100_hxml::configuration::Configuration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Configuration::parse();
    run_app(config).await
}
