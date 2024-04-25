use crate::websocket_types::RequestStruct;

use super::Public;

#[async_trait::async_trait]
pub trait EventHandler: Send + Sync {
    async fn on_join(&mut self, request: RequestStruct, public: &mut Public);
    async fn on_message(&mut self, public: &mut Public);
    async fn on_close(&mut self);
}
