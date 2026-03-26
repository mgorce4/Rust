

use clap::Parser;
use v080_html::Configuration;
use v080_html::run_app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Configuration::parse();
    run_app(config).await
}
