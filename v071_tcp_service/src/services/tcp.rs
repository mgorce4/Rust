use crate::interfaces::lexicon::Lexicon;
use crate::use_cases::VotingController;
use crate::storage::Storage;
use crate::service::Service;
use async_trait::async_trait;

pub struct TcpService<Store> {
    pub port: u16,
    pub lexicon: Lexicon,
    pub controller: VotingController<Store>,
}

#[async_trait]
impl<Store: Storage + Send + Sync + 'static> Service<Store> for TcpService<Store> {
    fn new(port: u16, lexicon: Lexicon, controller: VotingController<Store>) -> Self {
        TcpService { port, lexicon, controller }
    }

    async fn serve(&mut self) -> Result<(), anyhow::Error> {
        // À implémenter : boucle TCP (placeholder)
        println!("TCP service listening on port {} (non implémenté)", self.port);
        // Exemple de clonage du controller pour des tâches futures :
        // Controller is already thread-safe, no need to clone here.
        Ok(())
    }
}
