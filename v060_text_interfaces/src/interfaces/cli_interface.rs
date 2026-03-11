use crate::domain::{VotingController, VoteOutcome, Voter, Candidate, BallotPaper};
use crate::storage::Storage;

pub async fn handle_line<Store: Storage>(line: &str, controller: &mut VotingController<Store>) -> anyhow::Result<String> {
    let input = line.trim();
    if input.is_empty() {
        return Ok("Veuillez saisir une commande : voter <votant> <candidat>, voter <votant>, votants, scores".to_string());
    }
    let mut parts = input.split_whitespace();
    match parts.next() {
        Some("voter") => {
            let votant = parts.next();
            let candidat = parts.next();
            if votant.is_none() {
                return Ok("Veuillez indiquer le nom du votant (ex: voter Tux NixOS)".to_string());
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
            if let Some(_ballot) = ballot {
                let outcome = controller.vote(crate::use_cases::VoteForm {
                    voter: voter.0,
                    candidate: candidat.unwrap_or("").to_string(),
                }).await?;
                return Ok(show_vote_outcome(outcome));
            }
        }
        Some("votants") => {
            let machine = controller.get_voting_machine().await?;
            let voters = machine.get_voters();
            return Ok(show_attendence_sheet(&crate::domain::AttendanceSheet(voters.clone())));
        }
        Some("scores") => {
            let machine = controller.get_voting_machine().await?;
            let scoreboard = machine.get_scoreboard();
            return Ok(show_scoreboard(scoreboard));
        }
        _ => {
            return Ok("Commande invalide. Commandes valides : voter <votant> <candidat>, voter <votant>, votants, scores".to_string());
        }
    }
    Ok("".to_string())
}

pub fn show_vote_outcome(outcome: VoteOutcome) -> String {
    match outcome {
        VoteOutcome::AcceptedVote(v, c) => format!("{} a voté {}", v.0, c.0),
        VoteOutcome::BlankVote(v) => format!("{} a voté blanc", v.0),
        VoteOutcome::InvalidVote(v) => format!("{} a voté nul", v.0),
        VoteOutcome::HasAlreadyVoted(v) => format!("{} a déjà voté", v.0),
    }
}

pub fn show_scoreboard(scoreboard: &crate::domain::Scoreboard) -> String {
    let mut msg = String::from("Scores :");
    for (c, s) in &scoreboard.scores {
        msg.push_str(&format!("\n{}: {}", c.0, s.0));
    }
    msg.push_str(&format!("\nBlancs: {}", scoreboard.blank_votes.0));
    msg.push_str(&format!("\nNuls: {}", scoreboard.invalid_score.0));
    msg
}

pub fn show_attendence_sheet(attendence_sheet: &crate::domain::AttendanceSheet) -> String {
    let voters = &attendence_sheet.0;
    if voters.is_empty() {
        "Aucun votant pour l'instant.".to_string()
    } else {
        let mut msg = String::from("Liste des votants :");
        for v in voters {
            msg.push_str(&format!("\n- {}", v.0));
        }
        msg
    }
}
