use crate::taxonomy::{ContentString, ResponseData, Timeout, TransportError, TransportUrlVO};

#[async_trait::async_trait]
pub trait IHttpProviderPort: Send + Sync {
    async fn get(
        &self,
        url: TransportUrlVO,
        timeout: Option<Timeout>,
    ) -> Result<ResponseData, TransportError>;

    async fn post(
        &self,
        url: TransportUrlVO,
        body: ContentString,
        timeout: Option<Timeout>,
    ) -> Result<ResponseData, TransportError>;
}
