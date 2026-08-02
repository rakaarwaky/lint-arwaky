// PURPOSE: MaintenanceCommandsSurface — CLI surface for maintenance (doctor, security, dependencies)
// Adapted: uses std::process::Command for tool checks since MaintenanceCommandsAggregate
// methods are async and no tokio runtime is available in this crate.
use shared::common::ExitCode;

fn status_icon(is_ok: bool) -> &'static str {
    if std::env::var_os("NO_COLOR").is_some() {
        if is_ok { "[OK]  " } else { "[FAIL]" }
    } else {
        if is_ok { "✓" } else { "✗" }
    }
}

fn check_tool(name: &str, args: &[&str]) -> (String, String) {
    match std::process::Command::new(name).args(args).output() {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if out.status.success() {
                (stdout, "OK".to_string())
            } else {
                (stdout, "FAIL".to_string())
            }
        }
        Err(_) => (String::new(), "NOT FOUND".to_string()),
    }
}

pub fn handle_doctor() -> ExitCode {
    println!("Environment Diagnostics");
    println!();

    println!("Rust Toolchain:");
    let (ver, status) = check_tool("rustc", &["--version"]);
    println!(
        "  {} rustc {}  ({})",
        status_icon(status == "OK"),
        ver,
        status
    );
    let (ver, status) = check_tool("cargo", &["--version"]);
    println!(
        "  {} cargo {}  ({})",
        status_icon(status == "OK"),
        ver,
        status
    );

    println!();
    println!("Python Toolchain:");
    let (ver, status) = check_tool("python3", &["--version"]);
    println!(
        "  {} python3 {}  ({})",
        status_icon(status == "OK"),
        ver,
        status
    );
    let (ver, status) = check_tool("pip3", &["--version"]);
    println!(
        "  {} pip3 {}  ({})",
        status_icon(status == "OK"),
        ver,
        status
    );

    println!();
    println!("JavaScript Toolchain:");
    let (ver, status) = check_tool("node", &["--version"]);
    println!(
        "  {} node {}  ({})",
        status_icon(status == "OK"),
        ver,
        status
    );
    let (ver, status) = check_tool("npm", &["--version"]);
    println!(
        "  {} npm {}  ({})",
        status_icon(status == "OK"),
        ver,
        status
    );

    println!();
    println!("VCS:");
    let (ver, status) = check_tool("git", &["--version"]);
    println!(
        "  {} git {}  ({})",
        status_icon(status == "OK"),
        ver,
        status
    );

    ExitCode::OK
}

pub fn handle_security(path: Option<shared::common::FilePath>) -> ExitCode {
    let target = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    println!("Security Vulnerability Scan — {}", target);
    println!();

    // Try cargo audit for Rust projects
    let cargo_lock = std::path::Path::new(&target).join("Cargo.lock");
    if cargo_lock.exists() {
        println!("Language: Rust");
        println!("Tool: cargo-audit");
        let output = std::process::Command::new("cargo")
            .args(["audit", "--json"])
            .current_dir(&target)
            .output();
        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(&stdout) {
                    if let Some(vulns) = val
                        .get("vulnerabilities")
                        .and_then(|v| v.get("list"))
                        .and_then(|v| v.as_array())
                    {
                        println!("Findings: {}", vulns.len());
                        for v in vulns {
                            let id = v
                                .get("advisory")
                                .and_then(|a| a.get("id"))
                                .and_then(|i| i.as_str())
                                .unwrap_or("unknown");
                            let pkg = v
                                .get("package")
                                .and_then(|p| p.as_str())
                                .unwrap_or("unknown");
                            let sev = v
                                .get("advisory")
                                .and_then(|a| a.get("severity"))
                                .and_then(|s| s.as_str())
                                .unwrap_or("unknown");
                            println!("  {} {} {}", sev.to_uppercase(), id, pkg);
                        }
                    }
                } else {
                    println!("  (no structured output — is cargo-audit installed?)");
                }
            }
            Err(e) => {
                eprintln!("Error: cargo-audit not installed or failed: {e}");
                return ExitCode::PREREQUISITE_MISSING;
            }
        }
    } else {
        println!("No Cargo.lock found — skipping Rust security scan.");
    }

    ExitCode::OK
}

pub fn handle_dependencies(path: Option<shared::common::FilePath>) -> ExitCode {
    let target = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    println!("Dependency Report — {}", target);
    println!();

    // Parse Cargo.lock for Rust
    let cargo_lock = std::path::Path::new(&target).join("Cargo.lock");
    if cargo_lock.exists() {
        println!("Language: Rust");
        match std::fs::read_to_string(&cargo_lock) {
            Ok(content) => {
                // Count [[package]] entries
                let count = content.matches("[[package]]").count();
                println!("Dependencies: {} total", count);
                println!();
                // Print first few package names as a preview
                for line in content.lines().take(100) {
                    if let Some(name) = line.strip_prefix("name = ") {
                        let name = name.trim_matches('"');
                        println!("  {name}");
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading Cargo.lock: {e}");
                return ExitCode::RUNTIME_ERROR;
            }
        }
    } else {
        println!("No Cargo.lock found — cannot generate dependency report.");
    }

    ExitCode::OK
}
