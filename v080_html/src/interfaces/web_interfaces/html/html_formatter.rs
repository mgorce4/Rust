use maud::{html, Markup};
use crate::interfaces::web_interfaces::WebRoutes;
use crate::interfaces::lexicon::Lexicon;
use crate::domain::VotingMachine;

pub fn vote_form(routes: &WebRoutes, lexicon: &Lexicon) -> Markup {
    html! {
        h2 { (lexicon.prompt) }
        form method="post" action={(routes.vote)}
            hx-post={(routes.vote)}
            hx-target="#outcome"
            hx-swap="innerHTML"
        {
            label { (lexicon.voter)
                input type="text" name="voter";
            }
            label { (lexicon.candidate)
                input type="text" name="candidate";
            }
            button type="submit" { (lexicon.voted) }
        }
        p id="outcome" {}
    }
}

pub fn voting_machine(routes: &WebRoutes, lexicon: &Lexicon, machine: &VotingMachine) -> Markup {
    html! {
        div
            hx-get={(routes.results)}
            hx-trigger="every 3s"
        {
            h2 { (lexicon.scores) }
            ul {
                @for (candidate, score) in &machine.get_scoreboard().scores {
                    li { (candidate.0) " " (score.0) }
                }
            }
            h2 { (lexicon.list_voters) }
            ul {
                @for voter in machine.get_voters() {
                    li { (voter.0) }
                }
            }
        }
    }
}

pub fn index(routes: &WebRoutes, lexicon: &Lexicon, machine: &VotingMachine) -> Markup {
    html! {
        (maud::DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                title { (lexicon.prompt) }
                script src="https://unpkg.com/htmx.org@1.9.2" {}
            }
            body {
                h1 { (lexicon.prompt) }
                (vote_form(routes, lexicon))
                (voting_machine(routes, lexicon, machine))
            }
        }
    }
}
