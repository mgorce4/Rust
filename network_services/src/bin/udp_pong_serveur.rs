use tokio::net::UdpSocket;
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
    let server_address = format!("{}:{}", LOCALHOST, parameters.port);
    
    // Créer un socket UDP qui écoute sur le port spécifié
    let my_socket = UdpSocket::bind(&server_address).await?;
    println!("UDP server listening on {}", server_address);
    
    // Buffer pour recevoir les messages
    let mut buf = [0u8; 1024];
    
    // Boucle infinie pour traiter les requêtes séquentiellement
    loop {
        // Recevoir un message d'un client
        let (nb_bytes, sender_endpoint) = my_socket.recv_from(&mut buf).await?;
        
        // Afficher le message reçu
        let message = String::from_utf8_lossy(&buf[..nb_bytes]);
        println!("Received from {}: {}", sender_endpoint, message.trim());
        
        // Envoyer "PONG" en réponse au client
        my_socket.send_to(PONG.as_bytes(), &sender_endpoint).await?;
        println!("Sent PONG to {}", sender_endpoint);
    }
}
