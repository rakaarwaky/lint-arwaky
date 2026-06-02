/// stdio_transport_client — Direct subprocess execution transport.
use crate::contract::ICommandExecutorPort;
use crate::taxonomy::*;
use std::time::Duration;
use tokio::process::Command;
use async_trait::async_trait;
use std::collections::HashMap;

pub struct StdioClient {
    timeout: Duration,
}

impl StdioClient {
    pub fn new(timeout: Duration) -> Self {
        Self { timeout }
    }
}

#[async_trait]
impl ICommandExecutorPort for StdioClient {
    async fn execute_command(&self, command: PatternList, working_dir: FilePath, timeout: Option<Duration>) -> anyhow::Result<ResponseData> {
        let timeout_val = timeout.unwrap_or(self.timeout);
        let cmd_list: Vec<&str> = command.values.iter().map(|s| s.as_str()).collect();
        if cmd_list.is_empty() {
            anyhow::bail!("Empty command");
        }
        let mut cmd = Command::new(cmd_list[0]);
        if cmd_list.len() > 1 {
            cmd.args(&cmd_list[1..]);
        }
        cmd.current_dir(&working_dir.value).env("PYTHONUNBUFFERED", "1");
        cmd.kill_on_drop(true);

        let result = tokio::time::timeout(timeout_val, cmd.output()).await;
        match result {
            Ok(Ok(output)) => {
                let mut meta_map = HashMap::new();
                meta_map.insert("protocol".to_string(), serde_json::Value::String("Stdio".to_string()));
                
                Ok(ResponseData {
                    value: serde_json::Value::Null,
                    stdout: StdOutput::new(String::from_utf8_lossy(&output.stdout).to_string()),
                    stderr: StdError::new(String::from_utf8_lossy(&output.stderr).to_string()),
                    returncode: ExitCode::new(output.status.code().unwrap_or(-1) as i64),
                    metadata: MetadataVO::new(meta_map),
                })
            }
            Ok(Err(e)) => anyhow::bail!("Command execution failed: {}", e),
            Err(_) => {
                anyhow::bail!("Command timed out after {}s", timeout_val.as_secs())
            }
        }
    }

    async fn health_check(&self) -> anyhow::Result<ResponseData> {
        Ok(ResponseData::new(serde_json::json!({"status": "ok"})))
    }
}
