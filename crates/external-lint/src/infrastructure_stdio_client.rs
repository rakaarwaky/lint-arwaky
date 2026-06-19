// PURPOSE: StdioClient — ICommandExecutorPort implementation via stdio
use std::collections::HashMap;
use std::time::Duration;

use async_trait::async_trait;
use shared::cli_commands::contract_executor_port::ICommandExecutorPort;
use shared::mcp_server::taxonomy_job_vo::ResponseData;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::taxonomy_common_vo::PatternList;
use shared::taxonomy_duration_vo::Timeout;
use tokio::process::Command;

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
    async fn execute_command(
        &self,
        command: PatternList,
        working_dir: FilePath,
        timeout: Option<Timeout>,
    ) -> anyhow::Result<ResponseData> {
        let timeout_val = timeout
            .map(|d| Duration::from_secs(d.value() as u64))
            .unwrap_or(self.timeout);
        let cmd_list: Vec<&str> = command.values.iter().map(|s| s.as_str()).collect();
        if cmd_list.is_empty() {
            anyhow::bail!("Empty command");
        }
        let mut cmd = Command::new(cmd_list[0]);
        if cmd_list.len() > 1 {
            cmd.args(&cmd_list[1..]);
        }
        cmd.current_dir(&working_dir.value)
            .env("PYTHONUNBUFFERED", "1");
        cmd.kill_on_drop(true);

        let result = tokio::time::timeout(timeout_val, cmd.output()).await;
        match result {
            Ok(Ok(output)) => {
                let mut meta_map = HashMap::new();
                meta_map.insert(
                    "protocol".to_string(),
                    serde_json::Value::String("Stdio".to_string()),
                );
                Ok(ResponseData {
                    value: Some(serde_json::Value::Null),
                    stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                    stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                    returncode: output.status.code().unwrap_or(-1) as i64,
                    metadata: meta_map,
                })
            }
            Ok(Err(e)) => anyhow::bail!("Command execution failed: {}", e),
            Err(_) => anyhow::bail!("Command timed out after {}s", timeout_val.as_secs()),
        }
    }

    async fn health_check(&self) -> anyhow::Result<ResponseData> {
        Ok(ResponseData::new())
    }
}
