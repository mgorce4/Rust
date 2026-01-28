
use std::collections::{HashMap, HashSet};
use tokio::io::{self, AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut candidats = vec!["NixOS", "Ubuntu", "Debian"];
    let mut scores: HashMap<&str, usize> = candidats.iter().map(|&c| (c, 0)).collect();
    let mut scores_nuls = 0;
    let mut scores_blancs = 0;
    let mut votants: HashSet<String> = HashSet::new();
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
                let votant = votant.unwrap();
                if votants.contains(votant) {
                    println!("{} a déjà voté", votant);
                    continue;
                }
                votants.insert(votant.to_string());
                match candidat {
                    Some(c) => {
                        if !candidats.contains(&c) {
                            scores_nuls += 1;
                            println!("{} a voté nul", votant);
                        } else {
                            *scores.get_mut(c).unwrap() += 1;
                            println!("{} a voté {}", votant, c);
                        }
                    }
                    None => {
                        scores_blancs += 1;
                        println!("{} a voté blanc", votant);
                    }
                }
            }
            Some("votants") => {
                if votants.is_empty() {
                    println!("Aucun votant pour l'instant.");
                } else {
                    println!("Liste des votants :");
                    for v in &votants {
                        println!("- {}", v);
                    }
                }
            }
            Some("scores") => {
                println!("Scores :");
                for c in &candidats {
                    println!("{}: {}", c, scores[c]);
                }
                println!("Blancs: {}", scores_blancs);
                println!("Nuls: {}", scores_nuls);
            }
            _ => {
                println!("Commande invalide. Commandes valides : voter <votant> <candidat>, voter <votant>, votants, scores");
            }
        }
    }
    Ok(())
}
