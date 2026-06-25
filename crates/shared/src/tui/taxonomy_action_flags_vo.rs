#[derive(Debug, Clone)]
pub struct ActionFlags {
    pub git_diff: bool,
    pub dry_run: bool,
    pub threshold: u32,
    pub global_config: bool,
    pub use_sudo: bool,
    pub mcp_client: String,
}

impl Default for ActionFlags {
    fn default() -> Self {
        Self {
            git_diff: false,
            dry_run: false,
            threshold: 80,
            global_config: false,
            use_sudo: false,
            mcp_client: "claude".to_string(),
        }
    }
}

impl ActionFlags {
    pub fn toggle_git_diff(&mut self) {
        self.git_diff = !self.git_diff;
    }

    pub fn toggle_dry_run(&mut self) {
        self.dry_run = !self.dry_run;
    }

    pub fn toggle_global(&mut self) {
        self.global_config = !self.global_config;
    }

    pub fn toggle_sudo(&mut self) {
        self.use_sudo = !self.use_sudo;
    }

    pub fn set_threshold(&mut self, value: u32) {
        self.threshold = value;
    }

    pub fn set_mcp_client(&mut self, client: impl Into<String>) {
        self.mcp_client = client.into();
    }
}
