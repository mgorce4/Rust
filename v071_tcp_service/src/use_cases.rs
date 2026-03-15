use std::sync::Arc;
use tokio::sync::RwLock;
use crate::storage::Storage;
use crate::domain::{VotingMachine, BallotPaper, VoteOutcome};

pub struct VotingController<Store> {
  store: Arc<RwLock<Store>>,
}

impl<Store: Storage> VotingController<Store> {
  pub fn new(store: Store) -> Self {
    Self { store: Arc::new(RwLock::new(store)) }
  }

  pub async fn vote(&self, vote_form: VoteForm) -> anyhow::Result<VoteOutcome> {
    // Un seul verrou acquis pour toute la méthode
    let mut store = self.store.write().await;
    let mut machine = store.get_voting_machine().await?;
    let ballot_paper = BallotPaper::from(vote_form);
    let outcome = machine.vote(ballot_paper);
    store.put_voting_machine(machine).await?;
    Ok(outcome)
  }

  pub async fn get_voting_machine(&self) -> anyhow::Result<VotingMachine> {
    let store = self.store.read().await;
    store.get_voting_machine().await
  }
}
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct VoteForm {
  pub voter: String,
  pub candidate: String,
}