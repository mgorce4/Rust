use tokio::net::UdpSocket;
use clap::Parser;
use std::collections::HashMap;
use std::net::SocketAddr;

#[derive(Debug, Parser)]
struct Parameters {
    port: u16,
}

const LOCALHOST: &str = "127.0.0.1";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let parameters = Parameters::parse();
    let server_address = format!("{}:{}", LOCALHOST, parameters.port);
    
    let my_socket = UdpSocket::bind(&server_address).await?;
    println!("UDP chat server listening on {}", server_address);
    
    // Table pour stocker les clients : nom -> adresse
    let mut clients: HashMap<String, SocketAddr> = HashMap::new();
    
    let mut buf = [0u8; 1024];
    
    loop {
        let (nb_bytes, sender_addr) = my_socket.recv_from(&mut buf).await?;
        let message = String::from_utf8_lossy(&buf[..nb_bytes]).trim().to_string();
        
        // Vérifier si le client est déjà enregistré
        let client_name = clients.iter()
            .find(|(_, addr)| **addr == sender_addr)
            .map(|(name, _)| name.clone());
        
        if let Some(name) = client_name {
            // Client déjà identifié - traiter le message
            println!("Message from {}: {}", name, message);
            
            // Parser le format "destinataire : message" ou "dest1, dest2 : message"
            if let Some((dests_str, msg)) = message.split_once(':') {
                let msg = msg.trim();
                let destinations: Vec<&str> = dests_str.split(',')
                    .map(|s| s.trim())
                    .collect();
                
                // Envoyer à chaque destinataire
                let mut sent_count = 0;
                for dest_name in &destinations {
                    if let Some(&dest_addr) = clients.get(*dest_name) {
                        let response = format!("< {} : {}", name, msg);
                        my_socket.send_to(response.as_bytes(), dest_addr).await?;
                        sent_count += 1;
                        println!("Forwarded to {}", dest_name);
                    } else {
                        // Destinataire inconnu - envoyer erreur à l'expéditeur
                        let error = format!("Erreur : destinataire '{}' inconnu", dest_name);
                        my_socket.send_to(error.as_bytes(), sender_addr).await?;
                        println!("Unknown recipient: {}", dest_name);
                    }
                }
                
                if sent_count == 0 && destinations.len() > 0 {
                    println!("No valid recipients found");
                }
            } else {
                // Format invalide - envoyer erreur
                let error = "Erreur : format invalide. Utilisez 'destinataire : message'";
                my_socket.send_to(error.as_bytes(), sender_addr).await?;
                println!("Invalid message format from {}", name);
            }
        } else {
            // Nouveau client - phase d'identification
            let client_name = message.clone();
            
            // Vérifier si le nom est déjà pris
            if clients.contains_key(&client_name) {
                let error = format!("Erreur : nom '{}' déjà utilisé", client_name);
                my_socket.send_to(error.as_bytes(), sender_addr).await?;
                println!("Name '{}' already taken", client_name);
            } else {
                // Enregistrer le client
                clients.insert(client_name.clone(), sender_addr);
                let welcome = format!("Bienvenue, {} !", client_name);
                my_socket.send_to(welcome.as_bytes(), sender_addr).await?;
                println!("New client registered: {} from {}", client_name, sender_addr);
                println!("Connected clients: {:?}", clients.keys().collect::<Vec<_>>());
            }
        }
    }
}
