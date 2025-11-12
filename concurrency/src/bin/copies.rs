use clap::Parser;

#[derive(Debug, Parser)]
struct Parameters {
    n: usize,
}

async fn printhello(mut total: usize, i: usize) {
    total += 1; // Incrémente la copie locale de total
    println!("Bonjour {} (tâche n°{})", total, i);
    println!("Au revoir {} (tâche n°{})", total, i);
}

#[tokio::main]
async fn main() {
    let param = Parameters::parse();
    let total = 0_usize; // Entier initialisé à 0

    if param.n == 0 {
        println!("Veuillez fournir un nombre supérieur à zéro.");
        return;
    }
    
    for i in 0..param.n {
        let my_task = tokio::spawn(async move {
            printhello(total, i).await; // Passe total par valeur (copie)
        });
        my_task.await.unwrap();
    }
}