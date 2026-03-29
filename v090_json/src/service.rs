use async_trait::async_trait;
use crate::interfaces::lexicon::Lexicon;
use crate::domain::VotingController;

#[async_trait]
pub trait Service<Store: Clone> {
    fn new(port: u16, lexicon: Lexicon, controller: VotingController<Store>) -> Self;
    async fn serve(&mut self) -> Result<(), anyhow::Error>;
}
