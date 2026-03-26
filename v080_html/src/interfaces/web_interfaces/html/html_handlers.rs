use axum::{extract::{State, Form}, response::IntoResponse};
use crate::interfaces::web_interfaces::{AxumState, AxumError};
use crate::interfaces::web_interfaces::web_routes::WebRoutes;
use crate::interfaces::web_interfaces::html::html_formatter;
use crate::interfaces::lexicon::Lexicon;
use crate::interfaces::show_vote_outcome;
use crate::domain::{VotingMachine, VoteOutcome};
use crate::storage::Storage;

#[derive(serde::Deserialize)]
pub struct VoteForm {
    pub voter: String,
    pub candidate: String,
}

pub async fn get_index<Store: Storage>(
    State(app_state): State<AxumState<Store>>,
) -> Result<impl IntoResponse, AxumError> {
    let machine = app_state.controller.get_voting_machine().await?;
    Ok(html_formatter::index(&app_state.routes, &app_state.lexicon, &machine))
}

pub async fn get_results<Store: Storage>(
    State(app_state): State<AxumState<Store>>,
) -> Result<impl IntoResponse, AxumError> {
    let machine = app_state.controller.get_voting_machine().await?;
    Ok(html_formatter::voting_machine(&app_state.routes, &app_state.lexicon, &machine))
}

pub async fn vote<Store: Storage>(
    State(app_state): State<AxumState<Store>>,
    Form(vote_form): Form<VoteForm>,
) -> Result<impl IntoResponse, AxumError> {
    let outcome = app_state.controller.vote(vote_form).await?;
    Ok(show_vote_outcome(outcome, &app_state.lexicon))
}
