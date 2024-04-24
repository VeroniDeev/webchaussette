use crate::websocket_types::RequestStruct;

use super::Public;

#[async_trait::async_trait]
pub trait EventHandler: Send + Sync {
    async fn on_join(&self, request: RequestStruct);
    async fn on_message(&self, public: &mut Public);
    async fn on_close(&self);
}
