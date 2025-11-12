use clap::Parser;
use std::sync::Arc;

#[derive(Debug, Parser)]
struct Parameters {
    n: usize,
}

async fn printhello(stru: &str, i: usize) {
    println!("{} n°{}",stru, i);
    println!("Au revoir n°{}", i);
}

#[tokio::main]
async fn main() {
    let param = Parameters::parse();
    let stru = Arc::new(String::from("Bonjour")); // Allocation dynamique avec Arc pour partage
    if param.n == 0 {
        println!("Veuillez fournir un nombre supérieur à zéro.");
        return;
    }
    
    for i in 0..param.n {
        let stru_clone = Arc::clone(&stru); // Clone le pointeur Arc, pas la String
        let my_task = tokio::spawn(async move {
            printhello(&stru_clone, i).await;
        });
        my_task.await.unwrap();
    }
}