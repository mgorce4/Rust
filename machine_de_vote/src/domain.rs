use std::collections::BTreeMap as Map;
use std::collections::BTreeSet as Set;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Voter(pub String);

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Candidate(pub String);

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Score(pub usize);
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct AttendanceSheet(pub Set<Voter>);

impl AttendanceSheet{
    pub fn new() -> Self {
        return Self(Set::new());
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
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

#[derive(Clone)]
pub struct BallotPaper{
    pub voter: Voter,
    pub candidate: Option<Candidate>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum VoteOutcome{
    AcceptedVote(Voter, Candidate),
    BlankVote(Voter),
    InvalidVote(Voter),
    HasAlreadyVoted(Voter),
}
#[derive(Clone, PartialEq, Debug)]
pub struct VotingMachine {
    voters: AttendanceSheet,
    scoreboard: Scoreboard,
}
impl VotingMachine{
    pub fn new(candidates: Vec<Candidate>) -> Self {
        return Self{
            voters: AttendanceSheet::new(),
            scoreboard: Scoreboard::new(candidates),
        }
    }

    pub fn get_scoreboard(&self) -> &Scoreboard {
        return &self.scoreboard;
    }

    pub fn get_voters(&self) -> &AttendanceSheet {
        return &self.voters;
    }


    pub fn vote(&mut self, ballot_paper: BallotPaper) -> VoteOutcome{
        if self.voters.0.contains(&ballot_paper.voter){
            return VoteOutcome::HasAlreadyVoted(ballot_paper.voter)

        }else { 
            self.voters.0.insert(ballot_paper.voter.clone());
            match ballot_paper.candidate {
                None => { self.scoreboard.blank_votes = Score(self.scoreboard.blank_votes.0 + 1);
                            return VoteOutcome::BlankVote(ballot_paper.voter)},
                Some(candidate) => match self.scoreboard.scores.get_mut(&candidate){
                    None => { self.scoreboard.invalid_score = Score(self.scoreboard.invalid_score.0 + 1);
                                return VoteOutcome::InvalidVote(ballot_paper.voter)},
                    Some(score) => {
                        *score = Score(score.0 + 1);
                        return VoteOutcome::AcceptedVote(ballot_paper.voter, candidate);}
                }
            }
        }
    }
    
}



#[cfg(test)]
mod tests{
    use super::{VotingMachine, BallotPaper, Voter, VoteOutcome};

    #[test]
    fn one_can_vote_only_once() {
        let candidates = vec![];
        let mut voting_machine = VotingMachine::new(candidates);
        let voter = Voter(String::from("Tux"));
        let ballot_paper = BallotPaper {
            voter: voter.clone(),
            candidate: None,
        };
        let outcome = voting_machine.vote(ballot_paper.clone());
        assert_eq!(outcome, VoteOutcome::BlankVote(voter.clone()));
        let outcome2 = voting_machine.vote(ballot_paper);
        assert_eq!(outcome2, VoteOutcome::HasAlreadyVoted(voter));
    }
}