use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use clap::Parser;

#[derive(Debug, Parser)]
struct Parameters {
    port: u16,
}

const PONG: &str = "PONG\n";
const LOCALHOST: &str = "127.0.0.1";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let parameters = Parameters::parse();
    let listener = TcpListener::bind(format!("{}:{}", LOCALHOST, parameters.port)).await?;
    println!("TCP pong server listening on {}:{}", LOCALHOST, parameters.port);
    
    loop {
        let (stream, addr) = listener.accept().await?;
        println!("New connection from {}", addr);
        
        // Gérer chaque client dans une tâche séparée
        tokio::spawn(async move {
            let (reader, mut writer) = stream.into_split();
            let mut lines = BufReader::new(reader).lines();
            
            // Lire les messages du client
            while let Ok(Some(line)) = lines.next_line().await {
                println!("Received from {}: {}", addr, line);
                
                // Répondre "PONG" à chaque message reçu
                if let Err(e) = writer.write_all(PONG.as_bytes()).await {
                    eprintln!("Error writing to {}: {}", addr, e);
                    break;
                }
                
                if let Err(e) = writer.flush().await {
                    eprintln!("Error flushing to {}: {}", addr, e);
                    break;
                }
                
                println!("Sent PONG to {}", addr);
            }
            
            println!("Connection closed: {}", addr);
        });
    }
}
