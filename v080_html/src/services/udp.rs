use crate::interfaces::lexicon::Lexicon;
use crate::domain::VotingController;
use crate::storage::Storage;
use crate::service::Service;
use async_trait::async_trait;

pub struct UdpService<Store> {
    pub port: u16,
    pub lexicon: Lexicon,
    pub controller: VotingController<Store>,
}

#[async_trait]
impl<Store: Storage + Send + Sync> Service<Store> for UdpService<Store> {
    fn new(port: u16, lexicon: Lexicon, controller: VotingController<Store>) -> Self {
        UdpService { port, lexicon, controller }
    }

    async fn serve(&mut self) -> Result<(), anyhow::Error> {
        // À implémenter : boucle UDP (placeholder)
        println!("UDP service listening on port {} (non implémenté)", self.port);
        Ok(())
    }
}
