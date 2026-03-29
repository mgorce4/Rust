#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{VotingController, VotingMachine, Candidate};
    use crate::storages::memory::MemoryStore;
    use crate::interfaces::lexicons::french::FRENCH_LEXICON;
    use crate::storage::Storage;
    use tokio;

    async fn setup_controller_with_candidates(candidates: Vec<&str>) -> VotingController<MemoryStore> {
        let machine = VotingMachine::new(
            candidates.iter().map(|&name| Candidate(name.to_string())).collect()
        );
        let store = MemoryStore::new(machine).await.unwrap();
        VotingController::new(store)
    }

    #[tokio::test]
    async fn demande_commande_si_vide() {
        let mut controller = setup_controller_with_candidates(vec!["Alice"]).await;
        let res = handle_line("", &mut controller, &FRENCH_LEXICON).await.unwrap();
        assert_eq!(res, FRENCH_LEXICON.prompt);
    }

    #[tokio::test]
    async fn affiche_votants() {
        let mut controller = setup_controller_with_candidates(vec!["Alice"]).await;
        let _ = handle_line("voter Tux Alice", &mut controller, &FRENCH_LEXICON).await;
        let res = handle_line("votants", &mut controller, &FRENCH_LEXICON).await.unwrap();
        assert!(res.contains("Tux"));
    }

    #[tokio::test]
    async fn affiche_scores() {
        let mut controller = setup_controller_with_candidates(vec!["Alice"]).await;
        let _ = handle_line("voter Tux Alice", &mut controller, &FRENCH_LEXICON).await;
        let res = handle_line("scores", &mut controller, &FRENCH_LEXICON).await.unwrap();
        assert!(res.contains("Scores"));
        assert!(res.contains("Alice"));
    }

    #[tokio::test]
    async fn peut_voter() {
        let mut controller = setup_controller_with_candidates(vec!["Alice"]).await;
        let res = handle_line("voter Tux Alice", &mut controller, &FRENCH_LEXICON).await.unwrap();
        assert!(res.contains("Tux a voté Alice"));
    }

    #[tokio::test]
    async fn peut_voter_blanc() {
        let mut controller = setup_controller_with_candidates(vec!["Alice"]).await;
        let res = handle_line("voter Tux", &mut controller, &FRENCH_LEXICON).await.unwrap();
        assert!(res.contains("Tux a voté blanc"));
    }

    #[tokio::test]
    async fn demande_votant_si_manquant() {
        let mut controller = setup_controller_with_candidates(vec!["Alice"]).await;
        let res = handle_line("voter", &mut controller, &FRENCH_LEXICON).await.unwrap();
        assert_eq!(res, FRENCH_LEXICON.prompt_voter);
    }

    #[tokio::test]
    async fn commande_invalide() {
        let mut controller = setup_controller_with_candidates(vec!["Alice"]).await;
        let res = handle_line("foobar", &mut controller, &FRENCH_LEXICON).await.unwrap();
        assert_eq!(res, FRENCH_LEXICON.invalid_command);
    }
}

use crate::domain::{VotingController, Voter, Candidate, BallotPaper};
use crate::storage::Storage;
use crate::interfaces::lexicon::Lexicon;

pub async fn handle_line<Store: Storage + Clone>(line: &str, controller: &mut VotingController<Store>, lexicon: &Lexicon) -> anyhow::Result<String> {
    let input = line.trim();
    if input.is_empty() {
        return Ok(lexicon.prompt.to_string());
    }
    let mut parts = input.split_whitespace();
    match parts.next() {
        Some("voter") => {
            let votant = parts.next();
            let candidat = parts.next();
            if votant.is_none() {
                return Ok(lexicon.prompt_voter.to_string());
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
                let _ = controller.vote(crate::use_cases::VoteForm {
                    voter: voter.0,
                    candidate: candidat.unwrap_or("").to_string(),
                }).await?;
                // Removed unresolved show_vote_outcome call
            }
        }
        Some("votants") => {
            let machine = controller.get_voting_machine().await?;
            let voters = machine.get_voters();
            let voters_vec: Vec<_> = voters.collect();
            return Ok(show_attendence_sheet(&crate::domain::AttendanceSheet(voters_vec.into_iter().cloned().collect()), lexicon));
        }
        Some("scores") => {
            let machine = controller.get_voting_machine().await?;
            let scoreboard = machine.get_scoreboard();
            return Ok(show_scoreboard(scoreboard, lexicon));
        }
        _ => {
            return Ok(lexicon.invalid_command.to_string());
        }
    }
    Ok(String::new())
}


pub fn show_scoreboard(scoreboard: &crate::domain::Scoreboard, lexicon: &Lexicon) -> String {
    let mut msg = String::from(lexicon.scores);
    for (c, s) in &scoreboard.scores {
        msg.push_str(&format!("\n{}: {}", c.0, s.0));
    }
    msg.push_str(&format!("\n{}: {}", lexicon.blank_votes, scoreboard.blank_votes.0));
    msg.push_str(&format!("\n{}: {}", lexicon.null_votes, scoreboard.invalid_score.0));
    msg
}

pub fn show_attendence_sheet(attendence_sheet: &crate::domain::AttendanceSheet, lexicon: &Lexicon) -> String {
    let voters = &attendence_sheet.0;
    if voters.is_empty() {
        lexicon.no_voters.to_string()
    } else {
        let mut msg = String::from(lexicon.list_voters);
        for v in voters {
            msg.push_str(&format!("\n- {}", v.0));
        }
        msg
    }
}
