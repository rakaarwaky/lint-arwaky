use crate::cli_transport::taxonomy_protocol_vo::TransportProtocol;
use crate::cli_transport::taxonomy_protocol_vo::TransportUrlVO;
use crate::cli_transport::taxonomy_transport_error::TransportError;
use crate::http_client::contract_http_provider_port::IHttpProviderPort;
use crate::pipeline_jobs::taxonomy_job_vo::ResponseData;
use crate::shared_common::taxonomy_common_error::ErrorMessage;
use crate::shared_common::taxonomy_duration_vo::Timeout;
use crate::shared_common::taxonomy_source_vo::ContentString;
use std::collections::HashMap;
use std::time::Duration;

pub struct SyncHttpProvider {}

impl Default for SyncHttpProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl SyncHttpProvider {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl IHttpProviderPort for SyncHttpProvider {
    async fn get(
        &self,
        url: TransportUrlVO,
        timeout: Option<Timeout>,
    ) -> Result<ResponseData, TransportError> {
        let dur = timeout
            .map(|t| Duration::from_millis(t.value as u64))
            .unwrap_or(Duration::from_secs(2));
        let client = reqwest::blocking::Client::builder()
            .timeout(dur)
            .build()
            .map_err(|e| {
                TransportError::new(TransportProtocol::HTTP, ErrorMessage::new(e.to_string()))
            })?;
        let resp = client.get(&url.value).send().map_err(|e| {
            TransportError::new(TransportProtocol::HTTP, ErrorMessage::new(e.to_string()))
        })?;
        let text = resp.text().map_err(|e| {
            TransportError::new(TransportProtocol::HTTP, ErrorMessage::new(e.to_string()))
        })?;
        Ok(ResponseData {
            value: Some(serde_json::Value::String(text)),
            stdout: String::new(),
            stderr: String::new(),
            returncode: 0,
            metadata: HashMap::new(),
        })
    }

    async fn post(
        &self,
        url: TransportUrlVO,
        body: ContentString,
        timeout: Option<Timeout>,
    ) -> Result<ResponseData, TransportError> {
        let dur = timeout
            .map(|t| Duration::from_millis(t.value as u64))
            .unwrap_or(Duration::from_secs(2));
        let client = reqwest::blocking::Client::builder()
            .timeout(dur)
            .build()
            .map_err(|e| {
                TransportError::new(TransportProtocol::HTTP, ErrorMessage::new(e.to_string()))
            })?;
        let payload = body.value.clone();
        let resp = client.post(&url.value).body(payload).send().map_err(|e| {
            TransportError::new(TransportProtocol::HTTP, ErrorMessage::new(e.to_string()))
        })?;
        let text = resp.text().map_err(|e| {
            TransportError::new(TransportProtocol::HTTP, ErrorMessage::new(e.to_string()))
        })?;
        Ok(ResponseData {
            value: Some(serde_json::Value::String(text)),
            stdout: String::new(),
            stderr: String::new(),
            returncode: 0,
            metadata: HashMap::new(),
        })
    }
}
