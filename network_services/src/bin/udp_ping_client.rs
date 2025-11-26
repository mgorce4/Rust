use tokio::net::UdpSocket;
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
    
    // Créer un socket UDP qui écoute sur un port aléatoire
    let my_socket = UdpSocket::bind("127.0.0.1:0").await?;
    
    // Envoyer "PING" au serveur
    my_socket.send_to(PING.as_bytes(), &server_endpoint).await?;
    println!("Sent PING to {}", server_endpoint);
    
    // Recevoir la réponse dans le buffer
    let mut buf = [0u8; 1024];
    let (nb_bytes, sender_endpoint) = my_socket.recv_from(&mut buf).await?;
    
    // Afficher la réponse
    let response = String::from_utf8_lossy(&buf[..nb_bytes]);
    println!("Received from {}: {}", sender_endpoint, response.trim());
    
    Ok(())
}