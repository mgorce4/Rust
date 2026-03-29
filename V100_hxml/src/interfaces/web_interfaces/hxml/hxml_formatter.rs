use maud::{html, Markup};
use crate::interfaces::lexicon::Lexicon;
use crate::domain::VotingMachine;

// À adapter selon la définition réelle de MobileRoutes dans votre projet

use crate::interfaces::web_interfaces::web_routes::MobileRoutes;

// 1. Navigateur stack
pub fn index(routes: &MobileRoutes) -> Markup {
    html! {
        (maud::PreEscaped("<doc xmlns=\"https://hyperview.org/hyperview\">"))
        navigator id="navigator" type="stack" {
            nav-route id="home" href={(routes.home)};
        }
        (maud::PreEscaped("</doc>"))
    }
}

// 2. Écran d'accueil (vote)
pub fn home(routes: &MobileRoutes, lexicon: &Lexicon) -> Markup {
    html! {
        (maud::PreEscaped("<doc xmlns=\"https://hyperview.org/hyperview\">"))
        screen {
            styles {
                style id="title" fontSize="28" fontWeight="bold" marginBottom="16";
                style id="label" fontSize="16" marginBottom="4";
                style id="input" borderWidth="1" borderColor="#888" padding="8" marginBottom="8";
                style id="button" backgroundColor="red" color="white" fontSize="18" padding="10" marginBottom="16" textAlign="center";
                style id="button-green" backgroundColor="#228B22" color="white" fontSize="18" padding="10" marginBottom="8" textAlign="center";
                style id="container" padding="16";
                style id="info" fontSize="14" marginBottom="8";
            }
            body style="container" {
                text style="title" { (lexicon.prompt) }
                text style="label" { (lexicon.voter) }
                input style="input" value="";
                text style="label" { (lexicon.candidate) }
                input style="input" value="";
                form verb="post" action="replace" href={(routes.vote)} target="outcome" {
                    text style="button" { (lexicon.voted) }
                }
                text id="outcome" style="info" { "" }
                text style="button-green" href={(routes.voters)} { (lexicon.list_voters) }
                text style="button-green" href={(routes.scores)} { (lexicon.scores) }
                text style="button-green" action="reload" { "Rafraîchir" }
            }
        }
        (maud::PreEscaped("</doc>"))
    }
}

// 3. Écran votants
pub fn voters(lexicon: &Lexicon, machine: &VotingMachine) -> Markup {
    html! {
        (maud::PreEscaped("<doc xmlns=\"https://hyperview.org/hyperview\">"))
        screen {
            styles {
                style id="title" fontSize="28" fontWeight="bold" marginBottom="16";
                style id="votant" fontSize="18" marginBottom="4";
                style id="button-green" backgroundColor="#228B22" color="white" fontSize="18" padding="10" marginBottom="8" textAlign="center";
                style id="container" padding="16";
            }
            body style="container" {
                text style="title" { (lexicon.list_voters) }
                @for voter in machine.get_voters() {
                    text style="votant" { (voter) }
                }
                text style="button-green" action="back" { "Fermer" }
                text style="button-green" action="reload" { "Rafraîchir" }
            }
        }
        (maud::PreEscaped("</doc>"))
    }
}

// 4. Écran scores
pub fn scores(lexicon: &Lexicon, machine: &VotingMachine) -> Markup {
    html! {
        (maud::PreEscaped("<doc xmlns=\"https://hyperview.org/hyperview\">"))
        screen {
            styles {
                style id="title" fontSize="28" fontWeight="bold" marginBottom="16";
                style id="score" fontSize="18" marginBottom="4";
                style id="button-green" backgroundColor="#228B22" color="white" fontSize="18" padding="10" marginBottom="8" textAlign="center";
                style id="container" padding="16";
            }
            body style="container" {
                text style="title" { (lexicon.scores) }
                @for (candidate, score) in &machine.get_scoreboard().scores {
                    text style="score" { (candidate.0) ": " (score.0) }
                }
                text style="score" { (lexicon.blank_votes) ": 0" }
                text style="score" { (lexicon.null_votes) ": 0" }
                text style="button-green" action="back" { "Fermer" }
                text style="button-green" action="reload" { "Rafraîchir" }
            }
        }
        (maud::PreEscaped("</doc>"))
    }
}
