use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use clap::Parser;

#[derive(Debug, Parser)]
struct Parameters {
    host: String,
    port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let parameters = Parameters::parse();
    let server_endpoint = format!("{}:{}", parameters.host, parameters.port);
    
    // Se connecter au serveur TCP
    let stream = TcpStream::connect(&server_endpoint).await?;
    println!("Connected to {}", server_endpoint);
    println!("Type your messages (Ctrl+C to exit):");
    
    let (reader, mut writer) = stream.into_split();
    
    // Lire l'entrée standard
    let stdin = tokio::io::stdin();
    let mut stdin_reader = BufReader::new(stdin).lines();
    
    // Lire les réponses du serveur
    let mut server_reader = BufReader::new(reader).lines();
    
    loop {
        tokio::select! {
            // Lire une ligne depuis stdin
            line = stdin_reader.next_line() => {
                if let Ok(Some(input)) = line {
                    // Envoyer la ligne au serveur avec \n
                    let message = format!("{}\n", input);
                    writer.write_all(message.as_bytes()).await?;
                    writer.flush().await?;
                } else {
                    break; // EOF ou erreur
                }
            }
            
            // Recevoir une réponse du serveur
            line = server_reader.next_line() => {
                if let Ok(Some(response)) = line {
                    println!("{}", response);
                } else {
                    println!("Server disconnected");
                    break;
                }
            }
        }
    }
    
    Ok(())
}
