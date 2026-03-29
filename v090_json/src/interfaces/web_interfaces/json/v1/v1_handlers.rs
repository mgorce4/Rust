use axum::{
    extract::State,
    response::IntoResponse,
    Json,
};

use crate::interfaces::web_interfaces::{AxumError, AxumState};
use crate::interfaces::web_interfaces::json::v1::v1_formatter::{
    VoteFormV1,
    VoteOutcomeV1,
    VotingMachineV1,
};
use crate::storage::Storage;
use crate::use_cases::VoteForm;

pub async fn vote<Store: Storage + Clone>(
    State(mut app_state): State<AxumState<Store>>,
    Json(vote_form): Json<VoteFormV1>,
) -> Result<impl IntoResponse, AxumError> {
    let outcome = app_state.controller.vote(VoteForm::from(vote_form)).await?;
    Ok(Json(VoteOutcomeV1::from(outcome)))
}

pub async fn get_results<Store: Storage + Clone>(
    State(app_state): State<AxumState<Store>>,
) -> Result<impl IntoResponse, AxumError> {
    let machine = app_state.controller.get_voting_machine().await?;
    Ok(Json(VotingMachineV1::from(machine)))
}