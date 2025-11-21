use port_scanner::{is_open, Parameters};
use clap::Parser;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let my_parameters = Parameters::parse();
    
    use std::time::Instant;
    let instant = Instant::now();

    println!("Scanning {}:{}-{}", 
        my_parameters.host, 
        my_parameters.port_min, 
        my_parameters.port_max
    );
    
    // Tableau partagé pour stocker les ports ouverts
    let open_ports = Arc::new(Mutex::new(Vec::new()));
    let mut tasks = Vec::new();
    
    // Lancer une tâche par port
    for port in my_parameters.port_min..=my_parameters.port_max {
        let host = my_parameters.host.clone();
        let timeout = my_parameters.timeout;
        let open_ports_clone = Arc::clone(&open_ports);
        
        let task = tokio::spawn(async move {
            if is_open(&host, port, timeout).await {
                // Ajouter le port au tableau partagé
                let mut ports = open_ports_clone.lock().await;
                ports.push(port);
            }
        });
        
        tasks.push(task);
    }
    
    // Attendre que toutes les tâches se terminent
    for task in tasks {
        task.await.unwrap();
    }
    
    // Afficher tous les ports ouverts
    let ports = open_ports.lock().await;
    println!("\nOpen ports:");
    for port in ports.iter() {
        println!("  Port {}", port);
    }
    println!("Total: {} open port(s)", ports.len());
    
    println!("Scan complete");
    println!("{:?}", instant.elapsed());
}