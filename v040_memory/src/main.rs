

use clap::Parser;
use v040_memory::configuration::Configuration;
use v040_memory::app_builder::run_app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Configuration::parse();
    run_app(config).await
}
