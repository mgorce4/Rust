use std::fmt;
impl fmt::Display for Voter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Candidate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl VotingMachine {
    pub fn get_voters(&self) -> impl Iterator<Item = &Voter> {
        self.voters.0.iter()
    }
}
// Only one get_voters implementation should exist. If another exists elsewhere, remove it.
use crate::storage::Storage;
use crate::use_cases::VoteForm;


#[cfg(test)]
mod tests {
    use super::*;

    fn setup_machine() -> VotingMachine {
        let candidates = vec![Candidate("Alice".to_string()), Candidate("Bob".to_string())];
        VotingMachine::new(candidates)
    }

    #[test]
    fn test_accepted_vote() {
        let mut machine = setup_machine();
        let voter = Voter("Tux".to_string());
        let candidate = Candidate("Alice".to_string());
        let ballot = BallotPaper { voter: voter.clone(), candidate: Some(candidate.clone()) };
        let outcome = machine.vote(ballot);
        match outcome {
            VoteOutcome::AcceptedVote(v, c) => {
                assert_eq!(v, voter);
                assert_eq!(c, candidate);
            },
            _ => panic!("Expected AcceptedVote"),
        }
        // Vérifier le score du candidat
        let score = machine.get_scoreboard().scores.get(&candidate).unwrap().0;
        assert_eq!(score, 1);
    }

    #[test]
    fn test_blank_vote() {
        let mut machine = setup_machine();
        let voter = Voter("Tux".to_string());
        let ballot = BallotPaper { voter: voter.clone(), candidate: None };
        let outcome = machine.vote(ballot);
        match outcome {
            VoteOutcome::BlankVote(v) => assert_eq!(v, voter),
            _ => panic!("Expected BlankVote"),
        }
        // Vérifier le score blanc
        let blank_score = machine.get_scoreboard().blank_votes.0;
        assert_eq!(blank_score, 1);
    }

    #[test]
    fn test_invalid_vote() {
        let mut machine = setup_machine();
        let voter = Voter("Tux".to_string());
        let ballot = BallotPaper { voter: voter.clone(), candidate: Some(Candidate("NonExistent".to_string())) };
        let outcome = machine.vote(ballot);
        match outcome {
            VoteOutcome::InvalidVote(v) => assert_eq!(v, voter),
            _ => panic!("Expected InvalidVote"),
        }
        // Vérifier le score nul
        let invalid_score = machine.get_scoreboard().invalid_score.0;
        assert_eq!(invalid_score, 1);
    }

    #[test]
    fn test_has_already_voted() {
        let mut machine = setup_machine();
        let voter = Voter("Tux".to_string());
        let candidate = Candidate("Alice".to_string());
        let ballot = BallotPaper { voter: voter.clone(), candidate: Some(candidate.clone()) };
        let _ = machine.vote(ballot);
        // Deuxième vote du même votant
        let ballot2 = BallotPaper { voter: voter.clone(), candidate: Some(candidate.clone()) };
        let outcome = machine.vote(ballot2);
        match outcome {
            VoteOutcome::HasAlreadyVoted(v) => assert_eq!(v, voter),
            _ => panic!("Expected HasAlreadyVoted"),
        }
        // Le score du candidat ne doit pas avoir augmenté
        let score = machine.get_scoreboard().scores.get(&candidate).unwrap().0;
        assert_eq!(score, 1);
    }
}
use std::collections::BTreeMap as Map;
use std::collections::BTreeSet as Set;


#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Voter(pub String);

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Candidate(pub String);

#[derive(Clone)]
pub struct Score(pub usize);

#[derive(Clone)]
pub struct AttendanceSheet(pub Set<Voter>);

#[derive(Clone)]
pub struct Scoreboard {
    pub scores: Map<Candidate, Score>,
    pub blank_votes: Score,
    pub invalid_score: Score,
}

pub struct BallotPaper{
    pub voter: Voter,
    pub candidate: Option<Candidate>,
}

pub enum VoteOutcome{
    AcceptedVote(Voter, Candidate),
    BlankVote(Voter),
    InvalidVote(Voter),
    HasAlreadyVoted(Voter),
}

#[derive(Clone)]
pub struct VotingMachine {
    voters: AttendanceSheet,
    scoreboard: Scoreboard,
}

#[derive(Clone)]
pub struct VotingController<Store: Clone> {
    store: Store,
}

impl Scoreboard{
    pub fn new(candidates: Vec<Candidate>) -> Self {
        let scores = candidates.into_iter().map(|c| (c, Score(0))).collect();
        Scoreboard {
            scores,
            blank_votes: Score(0),
            invalid_score: Score(0),
        }
    }
}

impl VotingMachine{
    pub fn new(candidates: Vec<Candidate>) -> Self {
        VotingMachine {
            voters: AttendanceSheet(Set::new()),
            scoreboard: Scoreboard::new(candidates),
        }
    }
    pub fn recover_from(voters: AttendanceSheet, scoreboard: Scoreboard) -> Self {
        Self { voters, scoreboard }
    }
}

impl VotingMachine{
    pub fn vote(&mut self, ballot_paper: BallotPaper) -> VoteOutcome {
        if self.voters.0.contains(&ballot_paper.voter) {
            return VoteOutcome::HasAlreadyVoted(ballot_paper.voter);
        }
        self.voters.0.insert(ballot_paper.voter.clone());
        match ballot_paper.candidate {
            Some(candidate) => {
                if let Some(score) = self.scoreboard.scores.get_mut(&candidate) {
                    score.0 += 1;
                    VoteOutcome::AcceptedVote(ballot_paper.voter, candidate)
                } else {
                    self.scoreboard.invalid_score.0 += 1;
                    VoteOutcome::InvalidVote(ballot_paper.voter)
                }
            }
            None => {
                self.scoreboard.blank_votes.0 += 1;
                VoteOutcome::BlankVote(ballot_paper.voter)
            }
        }
    }
}

impl VotingMachine {
    pub fn get_scoreboard(&self) -> &Scoreboard {
        &self.scoreboard
    }
    // Only the iterator version of get_voters remains to avoid ambiguity
}

impl From<VoteForm> for BallotPaper {
    fn from(form: VoteForm) -> Self {
        BallotPaper {
            voter: Voter(form.voter),
            candidate: if form.candidate.is_empty() {
                None
            } else {
                Some(Candidate(form.candidate))
            },
        }
    }
}

impl<Store: Storage + Clone> VotingController<Store> {
  pub fn new(store: Store) -> Self{
    Self { store }
  }
  pub async fn vote(&mut self, vote_form: VoteForm) -> anyhow::Result<VoteOutcome> {
    let mut machine = self.store.get_voting_machine().await?;
    let ballot_paper = BallotPaper::from(vote_form);
    let outcome = machine.vote(ballot_paper);
    self.store.put_voting_machine(machine).await?;
    Ok(outcome)
  }
  pub async fn get_voting_machine(&self) -> anyhow::Result<VotingMachine> {
    self.store.get_voting_machine().await
  }

}



#[cfg(test)]
mod controller_tests {
    use super::*;
    use crate::domain::{Voter, Candidate, VoteOutcome, VotingMachine};
    use crate::storages::memory::MemoryStore;
    use crate::storage::Storage;
    use crate::domain::VotingController;


    async fn setup_controller_with_candidates(candidates: Vec<&str>) -> VotingController<MemoryStore> {
        let machine = VotingMachine::new(
            candidates.iter().map(|&name| Candidate(name.to_string())).collect()
        );
        let store = MemoryStore::new(machine).await.unwrap();
        VotingController::new(store)
    }


    #[tokio::test]
    async fn test_valid_vote() {
        let mut controller = setup_controller_with_candidates(vec!["Alice"]).await;
        let vote_form = VoteForm { voter: "Tux".to_string(), candidate: "Alice".to_string() };
        let outcome = controller.vote(vote_form).await.unwrap();
        assert!(matches!(outcome, VoteOutcome::AcceptedVote(ref v, ref c)
            if v == &Voter("Tux".to_string()) && c == &Candidate("Alice".to_string())));
        let machine = controller.get_voting_machine().await.unwrap();
        let score = machine.get_scoreboard().scores.get(&Candidate("Alice".to_string())).unwrap().0;
        assert_eq!(score, 1);
    }


    #[tokio::test]
    async fn test_blank_vote() {
        let mut controller = setup_controller_with_candidates(vec!["Alice"]).await;
        let vote_form = VoteForm { voter: "Tux".to_string(), candidate: "".to_string() };
        let outcome = controller.vote(vote_form).await.unwrap();
        assert!(matches!(outcome, VoteOutcome::BlankVote(ref v)
            if v == &Voter("Tux".to_string())));
        let machine = controller.get_voting_machine().await.unwrap();
        let score = machine.get_scoreboard().scores.get(&Candidate("Alice".to_string())).unwrap().0;
        assert_eq!(score, 0);
    }


    #[tokio::test]
    async fn test_invalid_vote() {
        let mut controller = setup_controller_with_candidates(vec!["Alice"]).await;
        let vote_form = VoteForm { voter: "Tux".to_string(), candidate: "Bob".to_string() };
        let outcome = controller.vote(vote_form).await.unwrap();
        assert!(matches!(outcome, VoteOutcome::InvalidVote(ref v)
            if v == &Voter("Tux".to_string())));
        let machine = controller.get_voting_machine().await.unwrap();
        let score = machine.get_scoreboard().scores.get(&Candidate("Alice".to_string())).unwrap().0;
        assert_eq!(score, 0);
    }


    #[tokio::test]
    async fn test_has_already_voted() {
        let mut controller = setup_controller_with_candidates(vec!["Alice"]).await;
        let vote_form = VoteForm { voter: "Tux".to_string(), candidate: "Alice".to_string() };
        let _ = controller.vote(vote_form.clone()).await.unwrap();
        // Deuxième vote du même votant
        let outcome = controller.vote(vote_form).await.unwrap();
        assert!(matches!(outcome, VoteOutcome::HasAlreadyVoted(ref v)
            if v == &Voter("Tux".to_string())));
        let machine = controller.get_voting_machine().await.unwrap();
        let score = machine.get_scoreboard().scores.get(&Candidate("Alice".to_string())).unwrap().0;
        assert_eq!(score, 1);
    }
}