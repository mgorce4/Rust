use tokio::net::UdpSocket;
use tokio::io::{AsyncBufReadExt, BufReader};
use clap::Parser;

#[derive(Debug, Parser)]
struct Parameters {
    /// Port local sur lequel écouter
    my_port: u16,
    /// Hôte distant du pair
    peer_host: String,
    /// Port distant du pair
    peer_port: u16,
}

const LOCALHOST: &str = "127.0.0.1";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let parameters = Parameters::parse();
    
    // Adresse locale pour écouter
    let my_address = format!("{}:{}", LOCALHOST, parameters.my_port);
    // Adresse du pair distant pour envoyer
    let peer_address = format!("{}:{}", parameters.peer_host, parameters.peer_port);
    
    // Créer un socket UDP qui écoute sur mon port
    let my_socket = UdpSocket::bind(&my_address).await?;
    println!("Chat peer listening on {}", my_address);
    println!("Connected to peer at {}", peer_address);
    println!("Type your messages (Ctrl+C to exit):");
    
    // Lire l'entrée standard
    let stdin = tokio::io::stdin();
    let mut stdin_reader = BufReader::new(stdin).lines();
    
    // Buffer pour recevoir les messages
    let mut buf = [0u8; 1024];
    
    loop {
        tokio::select! {
            // Lire une ligne depuis stdin et l'envoyer au pair
            line = stdin_reader.next_line() => {
                if let Ok(Some(input)) = line {
                    // Envoyer le message au pair distant
                    my_socket.send_to(input.as_bytes(), &peer_address).await?;
                } else {
                    break; // EOF
                }
            }
            
            // Recevoir un message du pair et l'afficher sur stdout
            result = my_socket.recv_from(&mut buf) => {
                if let Ok((nb_bytes, _sender)) = result {
                    let message = String::from_utf8_lossy(&buf[..nb_bytes]);
                    println!("{}", message);
                }
            }
        }
    }
    
    Ok(())
}
