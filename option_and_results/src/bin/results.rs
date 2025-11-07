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


fn main(){
    let sentence1 = "-17";
    let sentence2 = "Tux";

    convert_to_int1(sentence1);
    convert_to_int1(sentence2);

}