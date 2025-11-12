use clap::Parser;

#[derive(Debug, Parser)]
struct Parameters {
    n: usize,
}

async fn printhello(i: usize) {
    println!("Bonjour {}", i);
    println!("Au revoir {}", i);
}

#[tokio::main]
async fn main() {
    let param = Parameters::parse();
    if param.n == 0 {
        println!("Veuillez fournir un nombre supérieur à zéro.");
        return;
    }
    
    for i in 0..param.n {
        let my_task = tokio::spawn(async move {
            printhello(i).await;
        });
        my_task.await.unwrap();
    }
}