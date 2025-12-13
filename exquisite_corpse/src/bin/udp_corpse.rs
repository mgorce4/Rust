use tokio::net::UdpSocket;
use clap::Parser;
use exquisite_corpse::add_line_and_make_response;

#[derive(Debug, Parser)]
struct Parameters {
    port: u16,
}

const LOCALHOST: &str = "127.0.0.1";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let parameters = Parameters::parse();
    let server_address = format!("{}:{}", LOCALHOST, parameters.port);
    
    // Créer un socket UDP qui écoute sur le port spécifié
    let my_socket = UdpSocket::bind(&server_address).await?;
    println!("UDP exquisite corpse server listening on {}", server_address);
    println!("Clients can send lines to build a collaborative text");
    
    // Texte partagé entre tous les clients
    let mut text = String::new();
    
    // Buffer pour recevoir les messages
    let mut buf = [0u8; 1024];
    
    // Boucle infinie pour traiter les requêtes séquentiellement
    loop {
        // Recevoir un message d'un client
        let (nb_bytes, sender_endpoint) = my_socket.recv_from(&mut buf).await?;
        
        // Convertir le message reçu en String
        let line = String::from_utf8_lossy(&buf[..nb_bytes]).trim().to_string();
        println!("Received from {}: {}", sender_endpoint, line);
        
        // Appeler add_line_and_make_response avec le texte partagé
        let response = add_line_and_make_response(&line, &mut text);
        
        // Envoyer la réponse au client
        my_socket.send_to(response.as_bytes(), &sender_endpoint).await?;
        println!("Sent response to {}", sender_endpoint);
    }
}
