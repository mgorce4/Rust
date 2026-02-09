use std::collections::BTreeMap as Map;
use std::collections::BTreeSet as Set;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Voter(pub String);

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Candidate(pub String);

pub struct Score(pub usize);

pub struct AttendanceSheet(pub Set<Voter>);

pub struct Scoreboard {
    pub scores: Map<Candidate, Score>,
    pub blank_votes: Score,
    pub invalid_score: Score,
}

impl Scoreboard{
    pub fn new (candidates: Vec<Candidate>) -> Self {
        let mut scores = Map::new();
        for candidate in candidates{
            scores.insert(candidate, Score(0));
        }
        return Self{
            scores : scores,
            blank_votes: Score(0),
            invalid_score: Score(0),
        }
    }
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

pub struct VotingMachine {
    voters: AttendanceSheet,
    scoreboard: Scoreboard,
}

