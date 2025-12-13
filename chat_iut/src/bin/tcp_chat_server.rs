use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::mpsc;
use clap::Parser;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Parser)]
struct Parameters {
    port: u16,
}

const LOCALHOST: &str = "127.0.0.1";

type ClientsMap = Arc<Mutex<HashMap<String, mpsc::UnboundedSender<String>>>>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let parameters = Parameters::parse();
    let listener = TcpListener::bind(format!("{}:{}", LOCALHOST, parameters.port)).await?;
    println!("TCP chat server listening on {}:{}", LOCALHOST, parameters.port);
    
    // Map partagée : nom du client -> sender pour envoyer des messages
    let clients: ClientsMap = Arc::new(Mutex::new(HashMap::new()));
    
    loop {
        let (stream, addr) = listener.accept().await?;
        println!("New connection from {}", addr);
        
        let clients_clone = Arc::clone(&clients);
        
        tokio::spawn(async move {
            if let Err(e) = handle_client(stream, clients_clone).await {
                eprintln!("Error handling client: {}", e);
            }
        });
    }
}

async fn handle_client(stream: TcpStream, clients: ClientsMap) -> anyhow::Result<()> {
    let (reader, mut writer) = stream.into_split();
    let mut lines = BufReader::new(reader).lines();
    
    // Phase 1: Identification
    writer.write_all(b"Entrez votre nom: ").await?;
    writer.flush().await?;
    
    let client_name = match lines.next_line().await? {
        Some(name) => name.trim().to_string(),
        None => return Ok(()), // Connexion fermée
    };
    
    // Vérifier si le nom est déjà pris
    {
        let clients_lock = clients.lock().await;
        if clients_lock.contains_key(&client_name) {
            writer.write_all(format!("Erreur : nom '{}' déjà utilisé\n", client_name).as_bytes()).await?;
            writer.flush().await?;
            return Ok(());
        }
    }
    
    // Créer un canal pour ce client
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();
    
    // Enregistrer le client
    {
        let mut clients_lock = clients.lock().await;
        clients_lock.insert(client_name.clone(), tx);
        println!("Client '{}' registered", client_name);
        println!("Connected clients: {:?}", clients_lock.keys().collect::<Vec<_>>());
    }
    
    // Envoyer le message de bienvenue
    writer.write_all(format!("Bienvenue, {} !\n", client_name).as_bytes()).await?;
    writer.flush().await?;
    
    // Tâche pour envoyer les messages reçus via le canal au client
    let mut writer_clone = writer;
    let writer_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Err(e) = writer_clone.write_all(format!("{}\n", msg).as_bytes()).await {
                eprintln!("Error writing to client: {}", e);
                break;
            }
            if let Err(e) = writer_clone.flush().await {
                eprintln!("Error flushing: {}", e);
                break;
            }
        }
    });
    
    // Phase 2: Échanges de messages
    while let Ok(Some(line)) = lines.next_line().await {
        println!("Message from {}: {}", client_name, line);
        
        // Parser le format "destinataire : message" ou "dest1, dest2 : message"
        if let Some((dests_str, msg)) = line.split_once(':') {
            let msg = msg.trim();
            let destinations: Vec<&str> = dests_str.split(',')
                .map(|s| s.trim())
                .collect();
            
            // Envoyer à chaque destinataire
            let clients_lock = clients.lock().await;
            let mut sent_count = 0;
            
            for dest_name in &destinations {
                if let Some(dest_tx) = clients_lock.get(*dest_name) {
                    let response = format!("< {} : {}", client_name, msg);
                    if dest_tx.send(response).is_ok() {
                        sent_count += 1;
                        println!("Forwarded to {}", dest_name);
                    }
                } else {
                    // Destinataire inconnu - envoyer erreur à l'expéditeur
                    if let Some(sender_tx) = clients_lock.get(&client_name) {
                        let error = format!("Erreur : destinataire '{}' inconnu", dest_name);
                        let _ = sender_tx.send(error);
                    }
                    println!("Unknown recipient: {}", dest_name);
                }
            }
            
            if sent_count == 0 && destinations.len() > 0 {
                println!("No valid recipients found");
            }
        } else {
            // Format invalide - envoyer erreur
            let clients_lock = clients.lock().await;
            if let Some(sender_tx) = clients_lock.get(&client_name) {
                let error = "Erreur : format invalide. Utilisez 'destinataire : message'".to_string();
                let _ = sender_tx.send(error);
            }
            println!("Invalid message format from {}", client_name);
        }
    }
    
    // Déconnecter le client
    {
        let mut clients_lock = clients.lock().await;
        clients_lock.remove(&client_name);
        println!("Client '{}' disconnected", client_name);
        println!("Connected clients: {:?}", clients_lock.keys().collect::<Vec<_>>());
    }
    
    writer_task.abort();
    
    Ok(())
}
