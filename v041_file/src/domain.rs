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
    pub fn get_voters(&self) -> &Set<Voter> {
        &self.voters.0
    }
}

