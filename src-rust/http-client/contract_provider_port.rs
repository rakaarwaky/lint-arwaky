use crate::shared_common::taxonomy_source_vo::ContentString;
use /* UNKNOWN: ResponseData */ crate::pipeline_jobs::taxonomy_job_vo::ResponseData;
use crate::shared_common::taxonomy_duration_vo::Timeout;
use crate::cli_transport::taxonomy_client_error::TransportError;
use crate::cli_transport::taxonomy_protocol_vo::TransportUrlVO;

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
