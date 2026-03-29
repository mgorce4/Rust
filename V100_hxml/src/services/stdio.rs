use async_trait::async_trait;
use crate::interfaces::lexicon::Lexicon;
use crate::domain::VotingController;
use crate::storage::Storage;
use crate::interfaces::cli_interface::handle_line;
use crate::service::Service;
use tokio::io::{self, AsyncBufReadExt, BufReader};

pub struct StdioService<Store: Clone> {
    pub lexicon: Lexicon,
    pub controller: VotingController<Store>,
}

#[async_trait]
impl<Store: Storage + Send + Sync + Clone> Service<Store> for StdioService<Store> {
    fn new(_port: u16, lexicon: Lexicon, controller: VotingController<Store>) -> Self {
        StdioService { lexicon, controller }
    }

    async fn serve(&mut self) -> Result<(), anyhow::Error> {
        let stdin = BufReader::new(io::stdin());
        let mut lines = stdin.lines();
        println!("{}", self.lexicon.prompt);
        while let Some(line) = lines.next_line().await? {
            let msg = handle_line(&line, &mut self.controller, &self.lexicon).await?;
            println!("{}", msg);
        }
        Ok(())
    }
}
