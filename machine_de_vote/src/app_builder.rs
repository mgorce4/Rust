use crate::configuration::Configuration;
use crate::domain::{VotingMachine, BallotPaper, VoteOutcome, Candidate, Voter};
use tokio::io::{self, AsyncBufReadExt, BufReader};
use crate::storage;



pub async fn run_app(configuration:Configuration) -> anyhow::Result<()> {
    // You likely need to convert configuration.candidates (Vec<String>) to Vec<Candidate>
    let candidates = configuration.candidates.into_iter().map(|c| Candidate(c)).collect();
    let mut voting_machine = VotingMachine::new(candidates);
    let mut storage = storage::MemoryStorage::new(voting_machine.clone()).await?;
    
    let mut lines = BufReader::new(io::stdin()).lines();
    while let Some(line) = lines.next_line().await? {
        let mut words = line.split_whitespace();
        match words.next() {
            None => println!("Saisissez une commande svp! : voter | votants | scores"),
            Some(command) => {
                if command == "votants" {
                    println!("{:?}", voting_machine.get_voters());
                } else if command == "scores" {
                    println!("{:?}", voting_machine.get_scoreboard());
                } else if command == "voter" {
                    match words.next() {
                        None => println!("fournissez un votant svp!"),
                        Some(voter) => {
                            let ballot_paper = BallotPaper {
                                voter: Voter(voter.to_string()),
                                candidate: match words.next() {
                                    None => None,
                                    Some(candidate) => Some(Candidate(candidate.to_string())),
                                },
                            };
                            match voting_machine.vote(ballot_paper) {
                                VoteOutcome::AcceptedVote(voter, candidate) => println!("{} a voté pour {}", voter.0, candidate.0),
                                VoteOutcome::BlankVote(voter) => println!("{} a voté blanc", voter.0),
                                VoteOutcome::InvalidVote(voter) => println!("{} a voté pour un candidat invalide", voter.0),
                                VoteOutcome::HasAlreadyVoted(voter) => println!("{} a déjà voté", voter.0),
                            }
                        }
                    }
                } else {
                    println!("Commande inconnue! : voter | votants | scores");
                }
            }
        }
        println!("{}", line);
    }
    Ok(())
}