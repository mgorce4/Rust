use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use clap::Parser;
use exquisite_corpse::add_line_and_make_response;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Parser)]
struct Parameters {
    port: u16,
}

const LOCALHOST: &str = "127.0.0.1";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let parameters = Parameters::parse();
    let listener = TcpListener::bind(format!("{}:{}", LOCALHOST, parameters.port)).await?;
    println!("TCP exquisite corpse server listening on {}:{}", LOCALHOST, parameters.port);
    println!("Clients can send lines to build a collaborative text");
    
    // Texte partagé entre tous les clients avec Arc<Mutex>
    let shared_text = Arc::new(Mutex::new(String::new()));
    
    loop {
        let (stream, addr) = listener.accept().await?;
        println!("New connection from {}", addr);
        
        // Cloner l'Arc pour le passer à la tâche
        let text_clone = Arc::clone(&shared_text);
        
        // Créer une tâche pour chaque client
        tokio::spawn(async move {
            let (reader, mut writer) = stream.into_split();
            let mut lines = BufReader::new(reader).lines();
            
            // Lire et traiter chaque ligne reçue
            while let Ok(Some(line)) = lines.next_line().await {
                println!("Received from {}: {}", addr, line);
                
                // Acquérir le verrou et appeler add_line_and_make_response
                let response = {
                    let mut text = text_clone.lock().await;
                    add_line_and_make_response(&line, &mut text)
                };
                
                // Envoyer la réponse au client avec \n
                let response_with_newline = format!("{}\n", response);
                if let Err(e) = writer.write_all(response_with_newline.as_bytes()).await {
                    eprintln!("Error writing to {}: {}", addr, e);
                    break;
                }
                
                if let Err(e) = writer.flush().await {
                    eprintln!("Error flushing to {}: {}", addr, e);
                    break;
                }
            }
            
            println!("Connection closed: {}", addr);
            anyhow::Ok(())
        });
    }
}
