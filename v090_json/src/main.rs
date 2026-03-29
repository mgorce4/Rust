

use clap::Parser;
use v090_json::Configuration;
use v090_json::run_app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Configuration::parse();
    run_app(config).await
}
