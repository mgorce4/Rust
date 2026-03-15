// use tokio::io::{self, AsyncBufReadExt, BufReader};
use crate::configuration::Configuration;
use crate::domain::{VotingMachine};
use crate::storages::file::FileStore;
use crate::storages::memory::MemoryStore;
use crate::storage::Storage;
// use crate::interfaces::lexicon::Lexicon;
use crate::interfaces::lexicons::french::FRENCH_LEXICON;
use crate::interfaces::lexicons::english::ENGLISH_LEXICON;
use crate::domain::{Candidate, VotingController};
use crate::service::Service;
use crate::services::stdio::StdioService;
use crate::services::udp::UdpService;

pub fn create_voting_machine(configuration: &Configuration) -> VotingMachine {
    let candidates: Vec<Candidate> = configuration.candidates.iter().cloned().map(Candidate).collect();
    VotingMachine::new(candidates)
}

pub async fn handle_lines<Store, Serv>(config: Configuration) -> anyhow::Result<()>
where
    Store: Storage + Send + Sync + Clone,
    Serv: Service<Store> + Send,
{
    let initial_machine = create_voting_machine(&config);
    let store = Store::new(initial_machine).await?;
    let lexicon = match config.language.as_str() {
        "en" => ENGLISH_LEXICON,
        _ => FRENCH_LEXICON,
    };
    let port = config.port.unwrap_or(8888);
    let controller = VotingController::new(store);
    let mut service = Serv::new(port, lexicon, controller);
    service.serve().await
}

pub async fn dispatch_service<Store: Storage + Send + Sync + Clone>(config: Configuration) -> anyhow::Result<()> {
    match config.service.as_str() {
        "udp" => handle_lines::<Store, UdpService<Store>>(config).await,
        _ => handle_lines::<Store, StdioService<Store>>(config).await,
    }
}

pub async fn run_app(config: Configuration) -> anyhow::Result<()> {
    match config.storage {
        crate::configuration::StorageType::File => dispatch_service::<FileStore>(config).await,
        crate::configuration::StorageType::Memory => dispatch_service::<MemoryStore>(config).await,
    }
}
