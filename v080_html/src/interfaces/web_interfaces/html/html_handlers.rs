use axum::{extract::{State, Form}, response::IntoResponse};
use crate::interfaces::web_interfaces::{AxumState, AxumError};
// Removed unresolved and unused imports
use crate::storage::Storage;
use crate::interfaces::web_interfaces::html::html_formatter;
use crate::domain::VoteOutcome;
use crate::interfaces::lexicon::Lexicon;
pub fn show_vote_outcome(outcome: VoteOutcome, lexicon: &Lexicon) -> String {
    match outcome {
        VoteOutcome::AcceptedVote(v, c) => format!("{} {} {}", v.0, lexicon.voted, c.0),
        VoteOutcome::BlankVote(v) => format!("{} {}", v.0, lexicon.voted_blank),
        VoteOutcome::InvalidVote(v) => format!("{} {}", v.0, lexicon.voted_null),
        VoteOutcome::HasAlreadyVoted(v) => format!("{} {}", v.0, lexicon.already_voted),
    }
}

use crate::use_cases::VoteForm;

pub async fn get_index<Store: Storage + Clone>(
    State(app_state): State<AxumState<Store>>,
) -> Result<impl IntoResponse, AxumError> {
    let machine = app_state.controller.get_voting_machine().await?;
    Ok(html_formatter::index(&app_state.routes, &app_state.lexicon, &machine))
}

pub async fn get_results<Store: Storage + Clone>(
    State(app_state): State<AxumState<Store>>,
) -> Result<impl IntoResponse, AxumError> {
    let machine = app_state.controller.get_voting_machine().await?;
    Ok(html_formatter::voting_machine(&app_state.routes, &app_state.lexicon, &machine))
}

pub async fn vote<Store: Storage + Clone>(
    State(mut app_state): State<AxumState<Store>>,
    Form(vote_form): Form<VoteForm>,
) -> Result<impl IntoResponse, AxumError> {
    let outcome = app_state.controller.vote(vote_form).await?;
    Ok(show_vote_outcome(outcome, &app_state.lexicon))
}
