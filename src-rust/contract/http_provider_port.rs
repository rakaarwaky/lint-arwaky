use crate::taxonomy::{ContentString, ResponseData, Timeout, TransportUrlVO};

#[async_trait::async_trait]
pub trait IHttpProviderPort: Send + Sync {
    async fn get(
        &self,
        url: TransportUrlVO,
        timeout: Option<Timeout>,
    ) -> Result<ResponseData, String>;

    async fn post(
        &self,
        url: TransportUrlVO,
        body: ContentString,
        timeout: Option<Timeout>,
    ) -> Result<ResponseData, String>;
}
