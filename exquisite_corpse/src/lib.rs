pub fn add_line_and_make_response(line: &str, text : &mut String) -> String{
    //la fonction ajoute la ligne au texte, et renvoie le résultat encadré entre deux lignes délimiteurs faites en *
    if !text.is_empty() {
        text.push(' ');
    }
    text.push_str(line);
    let border = "*".repeat(30);
    format!("{}\n{}\n{}", border, text, border)
}