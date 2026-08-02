// PURPOSE: StdioClient — ICommandExecutorProtocol implementation via stdio (sync)
use std::collections::HashMap;
use std::process::Command;
use std::time::Duration;

use shared::common::taxonomy_common_vo::PatternList;
use shared::common::taxonomy_duration_vo::Timeout;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_response_data_vo::ResponseData;
use shared::external_lint::contract_executor_protocol::ICommandExecutorProtocol;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct StdioClient {
    timeout: Timeout,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ICommandExecutorProtocol for StdioClient {
    fn execute_command(
        &self,
        command: PatternList,
        working_dir: FilePath,
        timeout: Option<Timeout>,
    ) -> anyhow::Result<ResponseData> {
        // NOTE: std::process::Command is blocking; timeout is tracked for future
        // use with a thread-based timeout wrapper if needed.
        let _timeout_val = match timeout {
            Some(d) => Duration::from_secs_f64(d.value()),
            None => Duration::from_secs_f64(self.timeout.value()),
        };
        let cmd_list: Vec<&str> = command.values.iter().map(|s| s.as_str()).collect();
        if cmd_list.is_empty() {
            anyhow::bail!("Empty command");
        }
        let mut cmd = Command::new(cmd_list[0]);
        if cmd_list.len() > 1 {
            cmd.args(&cmd_list[1..]);
        }
        cmd.current_dir(working_dir.value())
            .env("PYTHONUNBUFFERED", "1");

        let result = cmd.output();
        match result {
            Ok(output) => {
                let mut meta_map = HashMap::new();
                meta_map.insert(
                    "protocol".to_string(),
                    serde_json::Value::String("Stdio".to_string()),
                );
                Ok(ResponseData {
                    value: Some(serde_json::Value::Null),
                    stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                    stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                    returncode: match output.status.code() {
                        Some(c) => c as i64,
                        None => -1,
                    },
                    metadata: meta_map,
                })
            }
            Err(e) => anyhow::bail!("Command execution failed: {}", e),
        }
    }

    fn health_check(&self) -> anyhow::Result<ResponseData> {
        Ok(ResponseData::new())
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl StdioClient {
    pub fn new(timeout: Timeout) -> Self {
        Self { timeout }
    }
}
