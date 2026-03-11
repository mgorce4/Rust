use super::super::lexicon::Lexicon;

pub const ENGLISH_LEXICON: Lexicon = Lexicon {
    blank: "blank",
    candidate: "candidate",
    voter: "voter",
    prompt: "Please enter a command: vote <voter> <candidate>, vote <voter>, voters, scores",
    prompt_voter: "Please specify the voter's name (e.g., vote Tux NixOS)",
    invalid_command: "Invalid command. Valid commands: vote <voter> <candidate>, vote <voter>, voters, scores",
    list_voters: "List of voters:",
    no_voters: "No voters yet.",
    scores: "Scores:",
    blank_votes: "Blank",
    null_votes: "Null",
    already_voted: "has already voted",
    voted: "voted",
    voted_blank: "voted blank",
    voted_null: "voted null",
};
