use clap::Parser;

#[derive(Debug, Parser)]
struct Parameters {
    n: usize,
}

async fn printhello(stru: &mut &str, i: usize) {
    //modifie stru
    *stru = "Ciao";
    println!("{} n°{}", *stru, i);
    println!("Au revoir n°{}", i);
}

#[tokio::main]
async fn main() {
    let param = Parameters::parse();
    let mut stru: &str = "Bonjour"; // Référence mutable vers un &str
    
    if param.n == 0 {
        println!("Veuillez fournir un nombre supérieur à zéro.");
        return;
    }
    
    for i in 0..param.n {
        // Modifie vers quelle string littérale la référence pointe
        stru = "Salut";
        
        // Appel direct sans spawn (car on ne peut pas passer &mut à travers spawn)
        printhello(&mut stru, i).await;
    }
}