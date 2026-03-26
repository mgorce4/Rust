use axum::{Router, routing::{get, post}};
use crate::interfaces::web_interfaces::AxumState;
use crate::interfaces::web_interfaces::web_routes::WebRoutes;
use crate::interfaces::web_interfaces::html::html_handlers;
use crate::storage::Storage;

pub fn make_router<Store: Storage + Clone + Send + Sync + 'static>(
    app_state: AxumState<Store>,
    routes: &WebRoutes,
) -> Router {
    Router::new()
        .route(routes.index, get(html_handlers::get_index::<Store>))
        .route(routes.vote, post(html_handlers::vote::<Store>))
        .route(routes.results, get(html_handlers::get_results::<Store>))
        .with_state(app_state)
}
