use async_trait::async_trait;
use crate::domain::VotingMachine;
use crate::storage::Storage;
use anyhow::Result;

#[derive(Clone)]
pub struct MemoryStore {
    machine: VotingMachine,
}

#[async_trait]
impl Storage for MemoryStore {
    async fn new(machine: VotingMachine) -> Result<Self> {
        Ok(MemoryStore { machine })
    }

    async fn get_voting_machine(&self) -> Result<VotingMachine> {
        Ok(self.machine.clone())
    }

    async fn put_voting_machine(&mut self, machine: VotingMachine) -> Result<()> {
        self.machine = machine;
        Ok(())
    }
}