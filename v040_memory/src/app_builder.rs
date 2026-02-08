
use tokio::io::{self, AsyncBufReadExt, BufReader};
use crate::configuration::Configuration;
use crate::domain::{VotingMachine, BallotPaper, Candidate, Voter, VoteOutcome};

pub fn create_voting_machine(configuration: &Configuration) -> VotingMachine {
    let candidates: Vec<Candidate> = configuration.candidates.iter().cloned().map(Candidate).collect();
    VotingMachine::new(candidates)
}

pub async fn run_app(config: Configuration) -> anyhow::Result<()> {
    let mut machine = create_voting_machine(&config);
    let stdin = BufReader::new(io::stdin());
    let mut lines = stdin.lines();

    println!("Bienvenue dans la machine de vote électronique !");
    println!("Commandes valides :\n- voter <votant> <candidat>\n- voter <votant>\n- votants\n- scores");

    while let Some(line) = lines.next_line().await? {
        let input = line.trim();
        if input.is_empty() {
            println!("Veuillez saisir une commande : voter <votant> <candidat>, voter <votant>, votants, scores");
            continue;
        }
        let mut parts = input.split_whitespace();
        match parts.next() {
            Some("voter") => {
                let votant = parts.next();
                let candidat = parts.next();
                if votant.is_none() {
                    println!("Veuillez indiquer le nom du votant (ex: voter Tux NixOS)");
                    continue;
                }
                let voter = Voter(votant.unwrap().to_string());
                let ballot = if let Some(c) = candidat {
                    Some(BallotPaper {
                        voter: voter.clone(),
                        candidate: Some(Candidate(c.to_string())),
                    })
                } else {
                    Some(BallotPaper {
                        voter: voter.clone(),
                        candidate: None,
                    })
                };
                if let Some(ballot) = ballot {
                    match machine.vote(ballot) {
                        VoteOutcome::AcceptedVote(v, c) => println!("{} a voté {}", v.0, c.0),
                        VoteOutcome::BlankVote(v) => println!("{} a voté blanc", v.0),
                        VoteOutcome::InvalidVote(v) => println!("{} a voté nul", v.0),
                        VoteOutcome::HasAlreadyVoted(v) => println!("{} a déjà voté", v.0),
                    }
                }
            }
            Some("votants") => {
                let voters = machine.get_voters();
                if voters.is_empty() {
                    println!("Aucun votant pour l'instant.");
                } else {
                    println!("Liste des votants :");
                    for v in voters {
                        println!("- {}", v.0);
                    }
                }
            }
            Some("scores") => {
                let scoreboard = machine.get_scoreboard();
                println!("Scores :");
                for (c, s) in &scoreboard.scores {
                    println!("{}: {}", c.0, s.0);
                }
                println!("Blancs: {}", scoreboard.blank_votes.0);
                println!("Nuls: {}", scoreboard.invalid_score.0);
            }
            _ => {
                println!("Commande invalide. Commandes valides : voter <votant> <candidat>, voter <votant>, votants, scores");
            }
        }
    }
    Ok(())
}
