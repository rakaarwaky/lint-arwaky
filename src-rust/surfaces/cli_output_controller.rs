use std::sync::Arc;
/// CLI output management utilities.
use std::sync::Mutex;




use crate::taxonomy::FilePath;







use crate::contract::service_container_aggregate::ServiceContainerAggregate;
pub struct OutputControllerSurface {
    pub container: Option<Arc<dyn ServiceContainerAggregate>>,
}

impl OutputControllerSurface {
    pub fn new() -> Self {
        Self { container: None }
    }

    pub fn get_output_dir(&self, ctx_output_dir: Option<&str>) -> Option<FilePath> {
        ctx_output_dir.map(|d| FilePath { value: d.to_string() })
            .or_else(|| {
                self.container.as_ref().and_then(|_c| {
                    None::<FilePath>
                })
            })
    }

    pub fn write_output(&self, output: &str, command: &str, fmt: Option<&str>) -> Option<FilePath> {
        let _ = output; // suppress unused
        let ext = fmt.unwrap_or("txt");
        let filename = format!("{}_{command}.{ext}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0));
        println!("[output] Would write to: {filename}");
        Some(FilePath { value: filename })
    }
}

// Lazy singleton
static INSTANCE: Mutex<Option<OutputControllerSurface>> = Mutex::new(None);

fn get_instance() -> std::sync::MutexGuard<'static, Option<OutputControllerSurface>> {
    let mut guard = INSTANCE.lock().expect("lock poisoned");
    if guard.is_none() {
        *guard = Some(OutputControllerSurface::new());
    }
    guard
}

pub fn get_output_dir(ctx_dir: Option<&str>) -> Option<FilePath> {
    let guard = get_instance();
    guard.as_ref().and_then(|s| s.get_output_dir(ctx_dir))
}

pub fn write_output(container: Option<&str>, output: &str, command: &str, fmt: Option<&str>) -> Option<FilePath> {
    let _ = container;
    let guard = get_instance();
    guard.as_ref().and_then(|s| s.write_output(output, command, fmt))
}

pub fn tee_stdout<F: FnOnce()>(_container: Option<&str>, f: F) -> String {
    f();
    String::new()
}

pub fn set_container(container: Arc<dyn ServiceContainerAggregate>) {
    let mut guard = INSTANCE.lock().expect("lock poisoned");
    if let Some(ref mut s) = *guard {
        s.container = Some(container);
    } else {
        *guard = Some(OutputControllerSurface { container: Some(container) });
    }
}
