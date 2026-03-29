use serde::{Deserialize, Serialize};
use std::collections::BTreeMap as Map;
use std::collections::BTreeSet as Set;

use crate::domain::{VoteOutcome, VotingMachine};
use crate::use_cases::VoteForm;

#[derive(Deserialize)]
pub struct VoteFormV1 {
    pub voter: String,
    pub candidate: String,
}

impl From<VoteFormV1> for VoteForm {
    fn from(value: VoteFormV1) -> Self {
        VoteForm {
            voter: value.voter,
            candidate: value.candidate,
        }
    }
}

#[derive(Serialize)]
pub enum VoteOutcomeV1 {
    AcceptedVote(String, String),
    HasAlreadyVoted(String),
    BlankVote(String),
    InvalidVote(String),
}

impl From<VoteOutcome> for VoteOutcomeV1 {
    fn from(value: VoteOutcome) -> Self {
        match value {
            VoteOutcome::AcceptedVote(voter, candidate) => {
                VoteOutcomeV1::AcceptedVote(voter.0, candidate.0)
            }
            VoteOutcome::HasAlreadyVoted(voter) => VoteOutcomeV1::HasAlreadyVoted(voter.0),
            VoteOutcome::BlankVote(voter) => VoteOutcomeV1::BlankVote(voter.0),
            VoteOutcome::InvalidVote(voter) => VoteOutcomeV1::InvalidVote(voter.0),
        }
    }
}

#[derive(Serialize)]
pub struct VotingMachineV1 {
    voters: Set<String>,
    scores: Map<String, usize>,
    blank_score: usize,
    invalid_score: usize,
}

impl From<VotingMachine> for VotingMachineV1 {
    fn from(value: VotingMachine) -> Self {
        let voters = value.get_voters().map(|voter| voter.0.clone()).collect();
        let scores = value
            .get_scoreboard()
            .scores
            .iter()
            .map(|(candidate, score)| (candidate.0.clone(), score.0))
            .collect();
        let blank_score = value.get_scoreboard().blank_votes.0;
        let invalid_score = value.get_scoreboard().invalid_score.0;

        VotingMachineV1 {
            voters,
            scores,
            blank_score,
            invalid_score,
        }
    }
}
