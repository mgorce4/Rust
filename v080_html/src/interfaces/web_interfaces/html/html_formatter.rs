use maud::{html, Markup};
use crate::interfaces::web_interfaces::WebRoutes;
use crate::interfaces::lexicon::Lexicon;
use crate::domain::VotingMachine;

pub fn vote_form(routes: &WebRoutes, lexicon: &Lexicon) -> Markup {
    html! {
        h2 { (lexicon.urn_title) }
        form method="post" action={(routes.vote)}
            hx-post={(routes.vote)}
            hx-target="#outcome"
            hx-swap="innerHTML"
        {
            label { (lexicon.voter_label)
                input type="text" name="voter";
            }
            label { (lexicon.candidate_label)
                input type="text" name="candidate";
            }
            button type="submit" { (lexicon.vote_button) }
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
            h2 { (lexicon.scores_title) }
            ul {
                @for (candidate, score) in &machine.scores {
                    li { (candidate) " " (score) }
                }
            }
            h2 { (lexicon.voters_title) }
            ul {
                @for voter in machine.get_voters() {
                    li { (voter) }
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
                title { (lexicon.page_title) }
                script src="https://unpkg.com/htmx.org@1.9.2" {}
            }
            body {
                h1 { (lexicon.page_title) }
                (vote_form(routes, lexicon))
                (voting_machine(routes, lexicon, machine))
            }
        }
    }
}
