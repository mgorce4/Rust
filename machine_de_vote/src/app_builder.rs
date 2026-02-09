use crate::configuration::Configuration;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use std::collections::BTreeSet as Set;
use std::collections::BTreeMap as Map;


pub async fn run_app(configuration:Configuration) -> anyhow::Result<()> {
    let mut voters = Set::new();
    let mut scores = Map::new();
    for candidate in configuration.candidates{
        scores.insert(candidate, 0);
    }

    
    let mut blank_votes = 0;
    let mut null_votes = 0;
    let mut lines = BufReader::new(io::stdin()).lines();
    while let Some(line) = lines.next_line().await?{

        let mut words = line.split_whitespace();
        match words.next(){
            None => println!("Saisissez une commande svp! : voter | votants | scores"),
            Some(command) => if command == "votants"{
                println!("{:?}", voters);
            }else if command == "scores" {
                println!("{:?}", scores);
            }else if command == "voter" {
                match words.next(){
                    None => println!("fournissez un votant svp!"),
                    Some(voter) => if voters.contains(voter){
                        println!("{} a déjà voté!", voter);

                    }else { 
                        voters.insert(voter.to_string());
                        match words.next(){
                            None => blank_votes += 1,
                            Some(candidate) => match scores.get_mut(candidate){
                                None => null_votes += 1,
                                Some(score) => *score += 1,
                            }
                        }
                    }
                }
            }    
            else {
                println!("Commande inconnue! : voter | votants | scores")
            }

        }
        println!("{}", line);   
    }
    return Ok(());
}