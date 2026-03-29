use axum::extract::{State, Multipart};
use crate::interfaces::web_interfaces::{AxumState, AxumError};
use crate::storage::Storage;
use crate::interfaces::web_interfaces::hxml::hxml_formatter;
use crate::use_cases::VoteForm;
use maud::html;

pub async fn get_index<Store: Storage + Clone>(
    State(app_state): State<AxumState<Store>>,
) -> Result<impl axum::response::IntoResponse, AxumError> {
    let markup = hxml_formatter::index(&app_state.routes.mobile);
    hxml_wrap(markup)
}

use axum::response::{IntoResponse, Response};
use maud::Markup;

/// Wraps Maud Markup in an Axum XML response for Hyperview
pub fn hxml_wrap(markup: Markup) -> Result<Response, AxumError> {
    let xml = markup.into_string();
    Ok(([("content-type", "application/xml; charset=utf-8")], xml).into_response())
}

/// Parses a multipart form into a VoteForm (stub implementation)
pub async fn parse_ballot_paper(mut multipart: Multipart) -> Result<VoteForm, AxumError> {
    use axum::extract::multipart::Field;
    use anyhow::anyhow;
    let mut voter: Option<String> = None;
    let mut candidate: Option<String> = None;

    while let Some(field_result) = multipart.next_field().await.transpose() {
        let mut field: Field = field_result.map_err(|e| AxumError::from(anyhow!(e)))?;
        let name = field.name().map(|s| s.to_string()).unwrap_or_default();
        let value = field.text().await.map_err(|e| AxumError::from(anyhow!(e)))?;
        match name.as_str() {
            "voter" => voter = Some(value),
            "candidate" => candidate = Some(value),
            _ => {}
        }
    }

    match (voter, candidate) {
        (Some(voter), Some(candidate)) => Ok(VoteForm { voter, candidate }),
        (None, _) => Err(AxumError::from(anyhow!("Missing 'voter' field in form"))),
        (_, None) => Err(AxumError::from(anyhow!("Missing 'candidate' field in form"))),
    }
}

pub async fn get_home<Store: Storage + Clone>(
    State(app_state): State<AxumState<Store>>,
) -> Result<impl axum::response::IntoResponse, AxumError> {
    let markup = hxml_formatter::home(&app_state.routes.mobile, &app_state.lexicon);
    hxml_wrap(markup)
}

pub async fn get_voters<Store: Storage + Clone>(
    State(app_state): State<AxumState<Store>>,
) -> Result<impl axum::response::IntoResponse, AxumError> {
    let machine = app_state.controller.get_voting_machine().await?;
    let markup = hxml_formatter::voters(&app_state.lexicon, &machine);
    hxml_wrap(markup)
}

pub async fn get_scores<Store: Storage + Clone>(
    State(app_state): State<AxumState<Store>>,
) -> Result<impl axum::response::IntoResponse, AxumError> {
    let machine = app_state.controller.get_voting_machine().await?;
    let markup = hxml_formatter::scores(&app_state.lexicon, &machine);
    hxml_wrap(markup)
}

pub async fn vote<Store: Storage + Clone>(
    State(mut app_state): State<AxumState<Store>>,
    vote_form_multipart: Multipart,
) -> Result<impl axum::response::IntoResponse, AxumError> {
    let vote_form = parse_ballot_paper(vote_form_multipart).await?;
    let outcome = app_state.controller.vote(vote_form).await?;
    use crate::interfaces::web_interfaces::html::html_handlers::show_vote_outcome;
    let outcome_str = show_vote_outcome(outcome, &app_state.lexicon);
    let markup = html! {
        (maud::PreEscaped("<text id=\"outcome\" xmlns=\"https://hyperview.org/hyperview\">"))
        (outcome_str)
        (maud::PreEscaped("</text>"))
    };
    hxml_wrap(markup)
}
