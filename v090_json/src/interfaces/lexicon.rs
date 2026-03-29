#[derive(PartialEq, Eq, Clone)]
pub struct Lexicon {
    pub blank: &'static str,
    pub candidate: &'static str,
    pub voter: &'static str,
    pub prompt: &'static str,
    pub prompt_voter: &'static str,
    pub invalid_command: &'static str,
    pub list_voters: &'static str,
    pub no_voters: &'static str,
    pub scores: &'static str,
    pub blank_votes: &'static str,
    pub null_votes: &'static str,
    pub already_voted: &'static str,
    pub voted: &'static str,
    pub voted_blank: &'static str,
    pub voted_null: &'static str,
}
