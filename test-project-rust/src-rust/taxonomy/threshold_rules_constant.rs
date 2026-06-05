// AES033 — constant-integrity-violation
// This `_constant` file contains forbidden constructs: fn definitions,
// impl blocks, struct definitions, and runtime initialization.
// _constant files must ONLY contain compile-time literals (pub const / pub static).

// ✅ Valid: compile-time constants
pub const MAX_RETRY_COUNT: u32 = 3;
pub const DEFAULT_TIMEOUT_SECS: u64 = 30;
pub static TOOL_NAME: &str = "lint-arwaky";

// ❌ FORBIDDEN: fn definition in a _constant file (AES033)
pub fn compute_threshold(score: f64) -> bool {
    score > 80.0
}

// ❌ FORBIDDEN: struct definition in a _constant file (AES033)
pub struct ThresholdConfig {
    pub min_score: f64,
    pub max_complexity: u32,
}

// ❌ FORBIDDEN: impl block in a _constant file (AES033)
impl ThresholdConfig {
    pub fn new() -> Self {
        Self {
            min_score: 80.0,
            max_complexity: 10,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.min_score > 0.0
    }
}

// ❌ FORBIDDEN: enum with fields in a _constant file (AES033)
pub enum SeverityLevel {
    Low(String),
    Medium { score: f64 },
    High(Vec<String>),
}

// ❌ FORBIDDEN: runtime initialization in a _constant file (AES033)
pub fn default_config() -> ThresholdConfig {
    ThresholdConfig::new()
}
