use tokio::io::{AsyncBufReadExt, BufReader};
use exquisite_corpse::add_line_and_make_response;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut text = String::new();
    
    println!("Exquisite Corpse - Enter lines (Ctrl+C to exit):");
    
    let stdin = tokio::io::stdin();
    let mut stdin_reader = BufReader::new(stdin).lines();
    
    // Lire les lignes depuis stdin
    while let Some(input) = stdin_reader.next_line().await? {
        // Appeler la fonction et afficher la réponse
        let response = add_line_and_make_response(&input, &mut text);
        println!("{}", response);
    }
    
    Ok(())
}
