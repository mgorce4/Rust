use tokio::net::UdpSocket;
use tokio::io::{AsyncBufReadExt, BufReader};
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
    
    // Créer un socket UDP qui écoute sur un port aléatoire
    let my_socket = UdpSocket::bind("127.0.0.1:0").await?;
    println!("Connected to {}", server_endpoint);
    println!("Type your messages (Ctrl+C to exit):");
    
    // Lire l'entrée standard ligne par ligne
    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin).lines();
    
    // Buffer pour recevoir les réponses
    let mut buf = [0u8; 1024];
    
    loop {
        tokio::select! {
            // Lire une ligne depuis stdin
            line = reader.next_line() => {
                if let Ok(Some(input)) = line {
                    // Envoyer la ligne au serveur
                    let message = format!("{}\n", input);
                    my_socket.send_to(message.as_bytes(), &server_endpoint).await?;
                } else {
                    break; // EOF ou erreur
                }
            }
            
            // Recevoir une réponse du serveur
            result = my_socket.recv_from(&mut buf) => {
                if let Ok((nb_bytes, _)) = result {
                    let response = String::from_utf8_lossy(&buf[..nb_bytes]);
                    print!("{}", response);
                }
            }
        }
    }
    
    Ok(())
}
