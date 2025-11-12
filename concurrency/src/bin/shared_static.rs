use clap::Parser;

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
    let stru = "Bonjour";
    if param.n == 0 {
        println!("Veuillez fournir un nombre supérieur à zéro.");
        return;
    }
    
    for i in 0..param.n {
        let my_task = tokio::spawn(async move {
            printhello(stru,i).await;
        });
        my_task.await.unwrap();
    }
}