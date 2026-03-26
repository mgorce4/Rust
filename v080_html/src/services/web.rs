use std::net::SocketAddr;
use async_trait::async_trait;
use crate::service::Service;
use crate::storage::Storage;
use axum::Router;
// TODO: Fix or re-add these imports if/when web_interfaces module is present
use crate::interfaces::web_interfaces::{AxumState, web_routes::WEB_ROUTES, router::make_router};
use crate::interfaces::lexicon::Lexicon;
use crate::domain::VotingController;

pub struct WebService {
  address: SocketAddr,
  router: Router,
}

#[async_trait]
impl<Store: Storage + Send + Sync + Clone + 'static> Service<Store> for WebService {
    fn new(port: u16, lexicon: Lexicon, controller: VotingController<Store>) -> Self {
        let address: SocketAddr = format!("127.0.0.1:{}", port).parse().unwrap();
        let app_state = AxumState {
            controller,
            routes: WEB_ROUTES.clone(),
            lexicon,
        };
        let router = make_router(app_state.clone(), &WEB_ROUTES);
        WebService { address, router }
    }

    async fn serve(&mut self) -> Result<(), anyhow::Error> {
        let listener = tokio::net::TcpListener::bind(&self.address).await.unwrap();
        axum::serve(listener, self.router.clone()).await.unwrap();
        Ok(())
    }
}
