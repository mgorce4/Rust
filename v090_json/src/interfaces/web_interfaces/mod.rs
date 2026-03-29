mod html;
pub mod json;
pub mod router;
pub mod web_routes;
use thiserror::Error;
use axum::{response::{IntoResponse, Response}, http::StatusCode};
use crate::domain::VotingController;
use crate::interfaces::lexicon::Lexicon;
use crate::interfaces::web_interfaces::web_routes::WebRoutes;

#[derive(Error, Debug)]
#[error("Error: {0}")]
pub struct AxumError(#[from] anyhow::Error);

impl IntoResponse for AxumError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self),
        )
        .into_response()
    }
}

#[derive(Clone)]
pub struct AxumState<Store: Clone> {
    pub controller: VotingController<Store>,
    pub routes: WebRoutes,
    pub lexicon: Lexicon,
}
