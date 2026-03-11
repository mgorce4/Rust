use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct VoteForm {
  pub voter: String,
  pub candidate: String,
}