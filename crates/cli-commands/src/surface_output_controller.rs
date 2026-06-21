// PURPOSE: OutputControllerSurface — CLI output management (tee, write output files)
use std::sync::Mutex;

use shared::source_parsing::taxonomy_path_vo::FilePath;

pub struct OutputControllerSurface {}

impl Default for OutputControllerSurface {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputControllerSurface {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_output_dir(&self, ctx_output_dir: Option<&str>) -> Option<FilePath> {
        ctx_output_dir.map(|d| FilePath {
            value: d.to_string(),
        })
    }

    pub fn write_output(&self, output: &str, command: &str, fmt: Option<&str>) -> Option<FilePath> {
        let _ = output;
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
    _container: Option<&str>,
    output: &str,
    command: &str,
    fmt: Option<&str>,
) -> Option<FilePath> {
    let guard = get_instance();
    guard
        .as_ref()
        .and_then(|s| s.write_output(output, command, fmt))
}

pub fn tee_stdout<F: FnOnce()>(_container: Option<&str>, f: F) -> String {
    f();
    String::new()
}
