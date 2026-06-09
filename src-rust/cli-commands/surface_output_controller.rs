use std::sync::Arc;
/// CLI output management utilities.
use std::sync::Mutex;

use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::source_parsing::taxonomy_path_vo::FilePath;

use crate::di_containers::contract_service_aggregate::ServiceContainerAggregate;
pub struct OutputControllerSurface {
    pub container: Option<Arc<dyn ServiceContainerAggregate>>,
}

impl OutputControllerSurface {
    pub fn new() -> Self {
        Self { container: None }
    }

    pub fn get_output_dir(&self, ctx_output_dir: Option<&str>) -> Option<FilePath> {
        ctx_output_dir
            .map(|d| FilePath {
                value: d.to_string(),
            })
            .or_else(|| self.container.as_ref().and_then(|_c| None::<FilePath>))
    }

    pub fn write_output(&self, output: &str, command: &str, fmt: Option<&str>) -> Option<FilePath> {
        let _ = output; // suppress unused
        let ext = fmt.unwrap_or("txt");
        let filename = format!(
            "{}_{command}.{ext}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0)
        );
        println!("[output] Would write to: {filename}");
        Some(FilePath { value: filename })
    }
}

// Lazy singleton
static INSTANCE: Mutex<Option<OutputControllerSurface>> = Mutex::new(None);

fn get_instance() -> std::sync::MutexGuard<'static, Option<OutputControllerSurface>> {
    let mut guard = INSTANCE.lock().unwrap_or_else(|e| e.into_inner());
    if guard.is_none() {
        *guard = Some(OutputControllerSurface::new());
    }
    guard
}

pub fn get_output_dir(ctx_dir: Option<&str>) -> Option<FilePath> {
    let guard = get_instance();
    guard.as_ref().and_then(|s| s.get_output_dir(ctx_dir))
}

pub fn write_output(
    container: Option<&str>,
    output: &str,
    command: &str,
    fmt: Option<&str>,
) -> Option<FilePath> {
    let _ = container;
    let guard = get_instance();
    guard
        .as_ref()
        .and_then(|s| s.write_output(output, command, fmt))
}

pub fn tee_stdout<F: FnOnce()>(_container: Option<&str>, f: F) -> String {
    f();
    String::new()
}

pub fn set_container(container: Arc<dyn ServiceContainerAggregate>) {
    let mut guard = INSTANCE.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(ref mut s) = *guard {
        s.container = Some(container);
    } else {
        *guard = Some(OutputControllerSurface {
            container: Some(container),
        });
    }
}

pub fn print_json(results: &[LintResult]) {
    match serde_json::to_string_pretty(results) {
        Ok(s) => println!("{}", s),
        Err(e) => eprintln!("[error] failed to serialize: {}", e),
    }
}

pub fn print_sarif(results: &[LintResult], target: &str) {
    let abs_target = std::path::Path::new(target)
        .canonicalize()
        .ok()
        .and_then(|p| p.to_str().map(|s| s.to_string()))
        .unwrap_or_else(|| target.to_string());

    let results_json: Vec<serde_json::Value> = results
        .iter()
        .map(|r| {
            let file_uri = if r.file.value().is_empty() {
                &abs_target
            } else {
                r.file.value()
            };
            serde_json::json!({
                "ruleId": r.code.to_string(),
                "level": match r.severity {
                    Severity::CRITICAL => "error",
                    Severity::HIGH => "error",
                    Severity::MEDIUM => "warning",
                    Severity::LOW | Severity::INFO => "note",
                },
                "message": { "text": r.message.value() },
                "locations": [{
                    "physicalLocation": {
                        "artifactLocation": { "uri": file_uri, "uriBaseId": "PROJECT_ROOT" },
                        "region": { "startLine": r.line.value(), "startColumn": r.column.value() }
                    }
                }]
            })
        })
        .collect();
    let sarif = serde_json::json!({
        "$schema": "https://json.schemastore.org/sarif-2.1.0.json",
        "version": "2.1.0",
        "runs": [{
            "tool": { "driver": { "name": "lint-arwaky", "version": env!("CARGO_PKG_VERSION") } },
            "originalUriBaseIds": {
                "PROJECT_ROOT": {
                    "uri": format!("file://{}/", abs_target)
                }
            },
            "results": results_json
        }]
    });
    match serde_json::to_string_pretty(&sarif) {
        Ok(s) => println!("{}", s),
        Err(e) => eprintln!("[error] failed to serialize SARIF: {}", e),
    }
}

pub fn print_junit(results: &[LintResult]) {
    let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    let errors_count = results
        .iter()
        .filter(|r| r.severity == Severity::CRITICAL)
        .count();
    let failures_count = results
        .iter()
        .filter(|r| matches!(r.severity, Severity::HIGH | Severity::MEDIUM))
        .count();
    xml.push_str(&format!(
        "<testsuite name=\"lint-arwaky\" tests=\"{}\" failures=\"{}\" errors=\"{}\">\n",
        results.len(),
        failures_count,
        errors_count
    ));
    for r in results {
        let safe = r.message.value().replace('"', "&quot;");
        xml.push_str(&format!(
            "  <testcase classname=\"{}\" name=\"{}\">\n",
            r.code.to_string(),
            safe
        ));
        match r.severity {
            Severity::CRITICAL => {
                xml.push_str(&format!("    <error message=\"{}\"/>\n", safe));
            }
            Severity::HIGH | Severity::MEDIUM => {
                xml.push_str(&format!("    <failure message=\"{}\"/>\n", safe));
            }
            Severity::LOW | Severity::INFO => {
                xml.push_str(&format!("    <skipped message=\"{}\"/>\n", safe));
            }
        }
        xml.push_str("  </testcase>\n");
    }
    xml.push_str("</testsuite>\n");
    println!("{}", xml);
}
