use clap::Parser;
mod configuration;
use configuration::Configuration;
mod app_builder;
use app_builder::run_app;
mod domain;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let configuration = Configuration::parse();

    return run_app(configuration).await;
}

