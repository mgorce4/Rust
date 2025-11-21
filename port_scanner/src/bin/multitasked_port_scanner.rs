use port_scanner::{is_open, Parameters};
use clap::Parser;

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
    
    let mut tasks = Vec::new();
    
    // Lancer une tâche par port
    for port in my_parameters.port_min..=my_parameters.port_max {
        let host = my_parameters.host.clone();
        let timeout = my_parameters.timeout;
        
        let task = tokio::spawn(async move {
            if is_open(&host, port, timeout).await {
                println!("Port {} is open", port);
            }
        });
        
        tasks.push(task);
    }
    
    // Attendre que toutes les tâches se terminent
    for task in tasks {
        task.await.unwrap();
    }
    
    println!("Scan complete");
    println!("{:?}", instant.elapsed());
}