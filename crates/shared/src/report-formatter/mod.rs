// report-formatter — contract types
pub mod contract_report_formatter_aggregate;
pub mod contract_report_formatter_protocol;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_report_formatter_aggregate::IReportFormatterAggregate;
pub use contract_report_formatter_protocol::IReportFormatterProtocol;
