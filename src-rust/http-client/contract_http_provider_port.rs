// PURPOSE: IHttpProviderPort — port trait for HTTP client operations
use crate::cli_transport::taxonomy_protocol_vo::TransportUrlVO;
use crate::cli_transport::taxonomy_transport_error::TransportError;
use crate::pipeline_jobs::taxonomy_job_vo::ResponseData;
use crate::shared_common::taxonomy_duration_vo::Timeout;
use crate::shared_common::taxonomy_source_vo::ContentString;

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
