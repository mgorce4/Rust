use clap::Parser;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Parser)]
struct Parameters {
    n: usize,
}

async fn printhello(counter: Arc<RwLock<usize>>, i: usize) {
    let mut num = counter.write().await; // Acquiert le verrou d'écriture
    *num += 1; // Incrémente le compteur
    println!("Bonjour {} (tâche n°{})", *num, i);
    println!("Au revoir {} (tâche n°{})", *num, i);
    // Le verrou est automatiquement libéré quand `num` sort du scope
}

#[tokio::main]
async fn main() {
    let param = Parameters::parse();
    let counter = Arc::new(RwLock::new(0_usize)); // Compteur partagé avec RwLock

    if param.n == 0 {
        println!("Veuillez fournir un nombre supérieur à zéro.");
        return;
    }
    
    for i in 0..param.n {
        let counter_clone = Arc::clone(&counter); // Clone le pointeur Arc
        let my_task = tokio::spawn(async move {
            printhello(counter_clone, i).await;
        });
        my_task.await.unwrap();
    }
}