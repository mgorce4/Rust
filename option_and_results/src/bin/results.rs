fn convert_to_int1(s: &str) {
    // tente de convertir la string en entier et d'afficher elle et son carré
    match s.parse::<i32>() {
        Ok(number) => {
            let square = number * number;
            println!("Le carré de {} est {}", number, square);
        }
        Err(_) => {
            println!("{} n'est pas un nombre entier", s);
        }
    }
}

fn convert_to_int2(s : &str){
    let number = s.parse::<i32>().expect("La chaine doit etre un nombre entier");
    let square = number * number;
    println!("Le carré de {} est {}", number, square);
}

fn convert_to_int3(s: &str) -> anyhow::Result<()> {
    let number = s.parse::<i32>()?;
    let square = number * number;
    println!("Le carré de {} est {}", number, square);
    Ok(())
}

fn main(){
    let sentence1 = "-17";
    let sentence2 = "Tux";

    convert_to_int1(sentence1);
    convert_to_int1(sentence2);

    convert_to_int2(sentence1);
    //convert_to_int2(sentence2); // Cela provoquera un panic

    match convert_to_int3(sentence1) {
        Ok(()) => (),
        Err(e) => println!("Erreur lors de la conversion de '{}': {}", sentence1, e),
    }
}