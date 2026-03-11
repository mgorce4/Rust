use tokio::io::{self, AsyncBufReadExt, BufReader};
use crate::configuration::Configuration;
use crate::domain::{VotingMachine};
use crate::storages::file::FileStore;
use crate::storages::memory::MemoryStore;
use crate::storage::Storage;
use crate::interfaces::lexicon::Lexicon;
use crate::interfaces::lexicons::french::FRENCH_LEXICON;
use crate::interfaces::lexicons::english::ENGLISH_LEXICON;
use crate::interfaces::cli_interface::handle_line;
use crate::domain::{Candidate, VotingController};

pub fn create_voting_machine(configuration: &Configuration) -> VotingMachine {
    let candidates: Vec<Candidate> = configuration.candidates.iter().cloned().map(Candidate).collect();
    VotingMachine::new(candidates)
}

pub async fn handle_lines<Store: Storage>(config: Configuration) -> anyhow::Result<()> {
    let initial_machine = create_voting_machine(&config);
    let mut store = Store::new(initial_machine).await?;
    let stdin = BufReader::new(io::stdin());
    let mut lines = stdin.lines();

    let lexicon: &Lexicon = match config.language.as_str() {
        "en" => &ENGLISH_LEXICON,
        _ => &FRENCH_LEXICON,
    };

    println!("{}", lexicon.prompt);

    let mut controller = VotingController::new(store);
    while let Some(line) = lines.next_line().await? {
        let msg = handle_line(&line, &mut controller, lexicon).await?;
        println!("{}", msg);
    }
    Ok(())
}

pub async fn run_app(config: Configuration) -> anyhow::Result<()> {
    match config.storage {
        crate::configuration::StorageType::File => handle_lines::<FileStore>(config).await,
        crate::configuration::StorageType::Memory => handle_lines::<MemoryStore>(config).await,
    }
}
