use super::super::lexicon::Lexicon;

pub const FRENCH_LEXICON: Lexicon = Lexicon {
    blank: "blanc",
    candidate: "candidat",
    voter: "votant",
    prompt: "Veuillez saisir une commande : voter <votant> <candidat>, voter <votant>, votants, scores",
    prompt_voter: "Veuillez indiquer le nom du votant (ex: voter Tux NixOS)",
    invalid_command: "Commande invalide. Commandes valides : voter <votant> <candidat>, voter <votant>, votants, scores",
    list_voters: "Liste des votants :",
    no_voters: "Aucun votant pour l'instant.",
    scores: "Scores :",
    blank_votes: "Blancs",
    null_votes: "Nuls",
    already_voted: "a déjà voté",
    voted: "a voté",
    voted_blank: "a voté blanc",
    voted_null: "a voté nul",
};
