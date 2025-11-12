use clap::Parser;

#[derive(Debug, Parser)]
struct Parameters {
    n: usize,
}

fn main(){
    let param = Parameters::parse();

    if param.n <= 0 {
        println!("Veuillez fournir un nombre supérieur à zéro.");
        return;
    }


    // Affichage séquentiel : Bonjour i, Au revoir i pour chaque i
    for i in 0..param.n {
        println!("Bonjour {}", i);
        println!("Au revoir {}", i);
    }
}