use crate::domain::VotingMachine;
use crate::storage::Storage;
use anyhow::Result;
use tokio::fs;
use serde_json;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap as Map, BTreeSet as Set};

pub const FILEPATH: &str = "machine.json";

#[derive(Serialize, Deserialize, Clone)]
struct ScoreboardDao {
    scores: Map<String, usize>,
    blank_score: usize,
    invalid_score: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct VotingMachineDao {
    voters: Set<String>,
    scoreboard: ScoreboardDao,
}

impl From<crate::domain::Scoreboard> for ScoreboardDao {
    fn from(scoreboard: crate::domain::Scoreboard) -> Self {
        ScoreboardDao {
            scores: scoreboard.scores.into_iter().map(|(c, s)| (c.0, s.0)).collect(),
            blank_score: scoreboard.blank_votes.0,
            invalid_score: scoreboard.invalid_score.0,
        }
    }
}

impl From<ScoreboardDao> for crate::domain::Scoreboard {
    fn from(dao: ScoreboardDao) -> Self {
        crate::domain::Scoreboard {
            scores: dao.scores.into_iter().map(|(c, s)| (crate::domain::Candidate(c), crate::domain::Score(s))).collect(),
            blank_votes: crate::domain::Score(dao.blank_score),
            invalid_score: crate::domain::Score(dao.invalid_score),
        }
    }
}

impl From<crate::domain::VotingMachine> for VotingMachineDao {
    fn from(machine: crate::domain::VotingMachine) -> Self {
        VotingMachineDao {
            voters: machine.get_voters().iter().map(|v| v.0.clone()).collect(),
            scoreboard: ScoreboardDao::from(machine.get_scoreboard().clone()),
        }
    }
}

impl From<VotingMachineDao> for crate::domain::VotingMachine {
    fn from(dao: VotingMachineDao) -> Self {
        crate::domain::VotingMachine::recover_from(
            crate::domain::AttendanceSheet(dao.voters.into_iter().map(crate::domain::Voter).collect()),
            dao.scoreboard.into(),
        )
    }
}

#[derive(Clone)]
pub struct FileStore {
    filepath: String,
}

impl FileStore {
    pub async fn create(machine: VotingMachine, filepath: &str) -> Result<Self> {
        if !std::path::Path::new(filepath).exists() {
            store_voting_machine(machine, filepath).await?;
        }
        Ok(FileStore { filepath: filepath.to_string() })
    }
}

#[async_trait::async_trait]
impl Storage for FileStore {
    async fn new(machine: VotingMachine) -> Result<Self> {
        FileStore::create(machine, FILEPATH).await
    }

    async fn get_voting_machine(&self) -> Result<VotingMachine> {
        let data = fs::read_to_string(&self.filepath).await?;
        let dao: VotingMachineDao = serde_json::from_str(&data)?;
        Ok(VotingMachine::from(dao))
    }

    async fn put_voting_machine(&mut self, machine: VotingMachine) -> Result<()> {
        let dao = VotingMachineDao::from(machine);
        let json = serde_json::to_string(&dao)?;
        fs::write(&self.filepath, json).await?;
        Ok(())
    }
}

async fn store_voting_machine(machine: VotingMachine, filepath: &str) -> anyhow::Result<()> {
    let dao = VotingMachineDao::from(machine);
    let json = serde_json::to_string(&dao)?;
    tokio::fs::write(filepath, json).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{VotingMachine, Candidate, AttendanceSheet, Scoreboard};
    use std::fs;
    use std::collections::BTreeSet;
    use tokio::runtime::Runtime;

    fn test_machine() -> VotingMachine {
        let candidates = vec![Candidate("A".to_string()), Candidate("B".to_string())];
        VotingMachine::new(candidates)
    }

    #[test]
    fn test_put_and_get_voting_machine() {
        let rt = Runtime::new().unwrap();
        let file = "test_machine1.json";
        let machine = test_machine();
        rt.block_on(async {
            let mut store = FileStore::create(machine, file).await.unwrap();
            let machine2 = test_machine();
            store.put_voting_machine(machine2).await.unwrap();
            let loaded = store.get_voting_machine().await.unwrap();
            assert_eq!(loaded.get_voters().len(), 0);
        });
        let _ = fs::remove_file(file);
    }

    #[test]
    fn test_file_persistence() {
        let rt = Runtime::new().unwrap();
        let file = "test_machine2.json";
        let machine = test_machine();
        rt.block_on(async {
            let mut store = FileStore::create(machine, file).await.unwrap();
            let machine2 = test_machine();
            store.put_voting_machine(machine2).await.unwrap();
        });
        // Nouvelle instance, même fichier
        rt.block_on(async {
            let store = FileStore::create(test_machine(), file).await.unwrap();
            let loaded = store.get_voting_machine().await.unwrap();
            assert_eq!(loaded.get_voters().len(), 0);
        });
        let _ = fs::remove_file(file);
    }
}