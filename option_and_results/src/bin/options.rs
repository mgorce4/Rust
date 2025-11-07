fn print_first_word(s: &str) {
    let first_word = s.split_whitespace().next();
    
    match first_word {
        Some(word) => println!("Premier mot : {}", word),
        None => println!("Chaîne vide"),
    }
}

fn print_first_word2(s: &str) {
    let first_word = s.split_whitespace().next().expect("La chaine doit etre non vide");
    println!("Premier mot : {}", first_word);
}

fn iterate_over_words(s : &str){
    for word in s.split_whitespace(){
        println!("Mot : {}", word);
    }
}

fn main(){
    let sentence1 = "Bonjour Limoges";
    let sentence2 = "";

    print_first_word(sentence1);
    print_first_word(sentence2);

    print_first_word2(sentence1);
    //print_first_word2(sentence2); // Cela provoquera un panic

    iterate_over_words(sentence1);
    iterate_over_words(sentence2);
}