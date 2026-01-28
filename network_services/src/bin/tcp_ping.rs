use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use clap::Parser;

#[derive(Debug, Parser)]
struct Parameters {
    host: String,
    port: u16,
}

const PING: &str = "PING\n";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let parameters = Parameters::parse();
    let server_endpoint = format!("{}:{}", parameters.host, parameters.port);
    
    // Se connecter au serveur TCP
    let mut stream = TcpStream::connect(&server_endpoint).await?;
    println!("Connected to {}", server_endpoint);
    
    // Envoyer "PING" au serveur
    stream.write_all(PING.as_bytes()).await?;
    stream.flush().await?;
    println!("Sent PING to {}", server_endpoint);
    
    // Lire la réponse ligne par ligne
    let (reader, _writer) = stream.into_split();
    let mut lines = BufReader::new(reader).lines();
    
    if let Ok(Some(response)) = lines.next_line().await {
        println!("Received: {}", response);
    }
    
    Ok(())
}
