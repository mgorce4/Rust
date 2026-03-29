use axum::{Router, routing::{get, post}};
use crate::interfaces::web_interfaces::AxumState;
use crate::interfaces::web_interfaces::web_routes::WebRoutes;
use crate::interfaces::web_interfaces::html::html_handlers;
use crate::interfaces::web_interfaces::json::v1::v1_handlers;
use crate::storage::Storage;

pub fn make_router<Store: Storage + Clone + Send + Sync + 'static>(
    app_state: AxumState<Store>,
    routes: &WebRoutes,
) -> Router {
    let v1_routes = Router::new()
        .route(routes.vote, post(v1_handlers::vote::<Store>))
        .route(routes.results, get(v1_handlers::get_results::<Store>));

    let json_routes = Router::new().nest(routes.v1, v1_routes);

    let hyperview_routes = Router::new()
        .route(&format!("{}/{}", routes.mobile.prefix, routes.mobile.index), get(crate::interfaces::web_interfaces::hxml::hxml_handlers::get_index::<Store>))
        .route(&format!("{}/{}", routes.mobile.prefix, routes.mobile.home), get(crate::interfaces::web_interfaces::hxml::hxml_handlers::get_home::<Store>))
        .route(&format!("{}/{}", routes.mobile.prefix, routes.mobile.scores), get(crate::interfaces::web_interfaces::hxml::hxml_handlers::get_scores::<Store>))
        .route(&format!("{}/{}", routes.mobile.prefix, routes.mobile.voters), get(crate::interfaces::web_interfaces::hxml::hxml_handlers::get_voters::<Store>))
        .route(&format!("{}/{}", routes.mobile.prefix, routes.mobile.vote), post(crate::interfaces::web_interfaces::hxml::hxml_handlers::vote::<Store>));

    Router::new()
        .route(routes.index, get(html_handlers::get_index::<Store>))
        .route(routes.vote, post(html_handlers::vote::<Store>))
        .route(routes.results, get(html_handlers::get_results::<Store>))
        .nest(routes.json, json_routes)
        .nest(routes.mobile.prefix, hyperview_routes)
        .with_state(app_state)
}

#[cfg(test)]
mod tests {
    use super::make_router;
    use async_trait::async_trait;
    use axum::body::{to_bytes, Body};
    use axum::http::{header::CONTENT_TYPE, Method, Request, StatusCode};
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use tower::util::ServiceExt;

    use crate::domain::{Candidate, VotingController, VotingMachine};
    use crate::interfaces::lexicons::ENGLISH_LEXICON;
    use crate::interfaces::web_interfaces::web_routes::WEB_ROUTES;
    use crate::interfaces::web_interfaces::AxumState;
    use crate::storage::Storage;

    #[derive(Clone)]
    struct TestStore {
        machine: Arc<Mutex<VotingMachine>>,
    }

    #[async_trait]
    impl Storage for TestStore {
        async fn new(machine: VotingMachine) -> anyhow::Result<Self> {
            Ok(Self {
                machine: Arc::new(Mutex::new(machine)),
            })
        }

        async fn get_voting_machine(&self) -> anyhow::Result<VotingMachine> {
            Ok(self.machine.lock().await.clone())
        }

        async fn put_voting_machine(&mut self, machine: VotingMachine) -> anyhow::Result<()> {
            *self.machine.lock().await = machine;
            Ok(())
        }
    }

    async fn make_test_router() -> axum::Router {
        let machine = VotingMachine::new(vec![Candidate("Alice".to_string())]);
        let store = TestStore::new(machine).await.unwrap();
        let controller = VotingController::new(store);
        let state = AxumState {
            controller,
            routes: WEB_ROUTES.clone(),
            lexicon: ENGLISH_LEXICON,
        };
        make_router(state, &WEB_ROUTES)
    }

    #[tokio::test]
    async fn html_vote_updates_results_score() {
        let app = make_test_router().await;

        let vote_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/vote")
                    .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
                    .body(Body::from("voter=Bob&candidate=Alice"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(vote_response.status(), StatusCode::OK);

        let results_response = app
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/results")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(results_response.status(), StatusCode::OK);
        let results_body = to_bytes(results_response.into_body(), usize::MAX)
            .await
            .unwrap();
        let html = String::from_utf8(results_body.to_vec()).unwrap();
        assert!(html.contains("Alice 1"));
    }

    #[tokio::test]
    async fn json_vote_updates_results_score() {
        let app = make_test_router().await;

        let vote_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/json/v1/vote")
                    .header(CONTENT_TYPE, "application/json")
                    .body(Body::from(r#"{"voter":"Bob","candidate":"Alice"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(vote_response.status(), StatusCode::OK);

        let vote_body = to_bytes(vote_response.into_body(), usize::MAX)
            .await
            .unwrap();
        let vote_json = String::from_utf8(vote_body.to_vec()).unwrap();
        assert!(vote_json.contains("AcceptedVote"));

        let results_response = app
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/json/v1/results")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(results_response.status(), StatusCode::OK);
        let results_body = to_bytes(results_response.into_body(), usize::MAX)
            .await
            .unwrap();
        let results_json = String::from_utf8(results_body.to_vec()).unwrap();
        assert!(results_json.contains("\"Alice\":1"));
        assert!(results_json.contains("\"Bob\""));
    }
}
