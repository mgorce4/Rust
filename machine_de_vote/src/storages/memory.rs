use crate::domain::VotingMachine;

pub struct MemoryStorage {
    machine: VotingMachine,
}

impl MemoryStorage {
    pub async fn new(machine: VotingMachine) -> anyhow::Result<Self> {
        return Ok(Self { machine });
    }

    pub async fn get_voting_machine(&self) -> anyhow::Result<VotingMachine> {
        return Ok(self.machine.clone());
    }

    pub async fn put_voting_machine(&mut self, machine: VotingMachine) -> anyhow::Result<()> {
        self.machine = machine;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{VotingMachine, Candidate};

    #[tokio::test]
    async fn test_put_and_get_voting_machine() {
        // Prépare une machine de vote initiale
        let candidates = vec![Candidate("Alice".to_string()), Candidate("Bob".to_string())];
        let machine1 = VotingMachine::new(candidates.clone());
        let mut storage = MemoryStorage::new(machine1.clone()).await.expect("init ok");

        // Modifie la machine de vote
        let machine2 = VotingMachine::new(candidates);
        storage.put_voting_machine(machine2.clone()).await.expect("put ok");

        // Récupère la machine stockée
        let got = storage.get_voting_machine().await.expect("get ok");
        assert_eq!(got, machine2);
    }
}