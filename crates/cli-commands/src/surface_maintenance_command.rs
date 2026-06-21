// PURPOSE: MaintenanceCommandsSurface — CLI surface for maintenance (doctor, security, dependencies)
use std::process::ExitCode;

use code_analysis::resolve_target;

pub struct MaintenanceCommandsSurface {}

impl Default for MaintenanceCommandsSurface {
    fn default() -> Self {
        Self::new()
    }
}

impl MaintenanceCommandsSurface {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn handle_doctor() {
    println!("Environment Diagnostics");
    println!();

    let check_tool = |name: &str, args: &[&str], required: bool| -> (&str, String) {
        let output = std::process::Command::new(name).args(args).output();
        match output {
            Ok(o) if o.status.success() => {
                let ver = String::from_utf8_lossy(&o.stdout)
                    .lines()
                    .next()
                    .unwrap_or("")
                    .trim()
                    .to_string();
                ("OK", ver)
            }
            _ => {
                if required {
                    ("FAIL", "NOT FOUND".to_string())
                } else {
                    ("WARN", "NOT FOUND".to_string())
                }
            }
        }
    };

    println!("Rust Toolchain:");
    let (cargo_st, cargo_ver) = check_tool("cargo", &["--version"], true);
    println!(
        "  {} cargo {}  ({})",
        if cargo_st == "OK" { "check" } else { "X" },
        cargo_ver,
        cargo_st
    );
    let (clippy_st, clippy_ver) = check_tool("cargo", &["clippy", "--version"], true);
    println!(
        "  {} clippy {}  ({})",
        if clippy_st == "OK" { "check" } else { "X" },
        clippy_ver,
        clippy_st
    );
    let (rustfmt_st, rustfmt_ver) = check_tool("rustfmt", &["--version"], true);
    println!(
        "  {} rustfmt {}  ({})",
        if rustfmt_st == "OK" { "check" } else { "X" },
        rustfmt_ver,
        rustfmt_st
    );
    if let Ok(p) = std::env::current_exe() {
        println!("  binary: {}", p.display());
    }

    println!();
    println!("Python Toolchain:");
    let (py_st, py_ver) = check_tool("python3", &["--version"], false);
    println!(
        "  {} python3 {}  ({})",
        if py_st == "OK" { "check" } else { "X" },
        py_ver,
        py_st
    );
    let (ruff_st, ruff_ver) = check_tool("ruff", &["--version"], false);
    println!(
        "  {} ruff {}  ({})",
        if ruff_st == "OK" { "check" } else { "X" },
        ruff_ver,
        ruff_st
    );
    let (mypy_st, mypy_ver) = check_tool("mypy", &["--version"], false);
    println!(
        "  {} mypy {}  ({})",
        if mypy_st == "OK" { "check" } else { "X" },
        mypy_ver,
        mypy_st
    );
    let (bandit_st, bandit_ver) = check_tool("bandit", &["--version"], false);
    println!(
        "  {} bandit {}  ({})",
        if bandit_st == "OK" { "check" } else { "X" },
        bandit_ver,
        bandit_st
    );

    println!();
    println!("JavaScript Toolchain:");
    let (node_st, node_ver) = check_tool("node", &["--version"], false);
    println!(
        "  {} node {}  ({})",
        if node_st == "OK" { "check" } else { "X" },
        node_ver,
        node_st
    );

    let eslint_local = std::path::Path::new("node_modules/.bin/eslint");
    if eslint_local.exists() {
        println!("  check eslint (local)");
    } else {
        let (es_st, es_ver) = check_tool("eslint", &["--version"], false);
        println!(
            "  {} eslint {}  ({})",
            if es_st == "OK" { "check" } else { "X" },
            es_ver,
            es_st
        );
    }

    let prettier_local = std::path::Path::new("node_modules/.bin/prettier");
    if prettier_local.exists() {
        println!("  check prettier (local)");
    } else {
        let (pr_st, pr_ver) = check_tool("prettier", &["--version"], false);
        println!(
            "  {} prettier {}  ({})",
            if pr_st == "OK" { "check" } else { "X" },
            pr_ver,
            pr_st
        );
    }

    let tsc_local = std::path::Path::new("node_modules/.bin/tsc");
    if tsc_local.exists() {
        println!("  check tsc (local)");
    } else {
        let (tsc_st, tsc_ver) = check_tool("tsc", &["--version"], false);
        println!(
            "  {} tsc {}  ({})",
            if tsc_st == "OK" { "check" } else { "X" },
            tsc_ver,
            tsc_st
        );
    }

    println!();
    println!("VCS:");
    let (git_st, git_ver) = check_tool("git", &["--version"], true);
    println!(
        "  {} git {}  ({})",
        if git_st == "OK" { "check" } else { "X" },
        git_ver,
        git_st
    );
    let (jj_st, jj_ver) = check_tool("jj", &["--version"], false);
    println!(
        "  {} jj {}  ({})",
        if jj_st == "OK" { "check" } else { "X" },
        jj_ver,
        jj_st
    );
}

pub fn handle_security(path: Option<String>) -> ExitCode {
    let root = resolve_target(path);
    println!("Security Vulnerability Scan — {}", root);
    println!();

    let cargo_lock = std::path::Path::new(&root).join("Cargo.lock");
    if cargo_lock.exists() {
        println!("Language: Rust");
        println!("Tool: cargo-audit");
        match std::process::Command::new("cargo")
            .args(["audit", "--json"])
            .output()
        {
            Ok(o) if o.status.success() => {
                let s = String::from_utf8_lossy(&o.stdout);
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&s) {
                    let advisories = json
                        .get("vulnerabilities")
                        .and_then(|v| v.get("list"))
                        .and_then(|l| l.as_array())
                        .map(|a| a.len())
                        .unwrap_or(0);
                    println!("Vulnerabilities found: {}", advisories);
                    if let Some(list) = json
                        .get("vulnerabilities")
                        .and_then(|v| v.get("list"))
                        .and_then(|l| l.as_array())
                    {
                        for adv in list {
                            let pkg = adv
                                .get("package")
                                .and_then(|p| p.get("name"))
                                .and_then(|n| n.as_str())
                                .unwrap_or("unknown");
                            let severity = adv
                                .get("severity")
                                .and_then(|s| s.as_str())
                                .unwrap_or("unknown");
                            let cve = adv
                                .get("advisory")
                                .and_then(|a| a.get("id"))
                                .and_then(|i| i.as_str())
                                .unwrap_or("unknown");
                            println!(
                                "  {} {} CVE: {} ({})",
                                severity.to_uppercase(),
                                pkg,
                                cve,
                                severity
                            );
                        }
                    }
                } else {
                    println!("No vulnerabilities found.");
                }
            }
            _ => {
                println!("cargo-audit not available. Install with: cargo install cargo-audit");
            }
        }
    } else {
        println!("Language: Python");
        println!("Tool: bandit");
        match std::process::Command::new("bandit")
            .args(["-r", "--format", "json", &root])
            .output()
        {
            Ok(o) if o.status.success() => {
                let s = String::from_utf8_lossy(&o.stdout);
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&s) {
                    let results_count = json
                        .get("results")
                        .and_then(|r| r.as_array())
                        .map(|a| a.len())
                        .unwrap_or(0);
                    println!("Findings: {}", results_count);
                    if let Some(results) = json.get("results").and_then(|r| r.as_array()) {
                        for r in results.iter().take(20) {
                            let test_id = r.get("test_id").and_then(|t| t.as_str()).unwrap_or("");
                            let issue = r.get("issue_text").and_then(|t| t.as_str()).unwrap_or("");
                            let sev = r
                                .get("issue_severity")
                                .and_then(|s| s.as_str())
                                .unwrap_or("");
                            let fname = r.get("filename").and_then(|f| f.as_str()).unwrap_or("");
                            let line = r.get("line_number").and_then(|l| l.as_u64()).unwrap_or(0);
                            println!(
                                "  {} {} {}:{} {}",
                                sev.to_uppercase(),
                                test_id,
                                fname,
                                line,
                                issue
                            );
                        }
                    }
                } else {
                    println!("No security issues found.");
                }
            }
            _ => {
                println!("bandit not available. Install with: pip install bandit");
            }
        }
    }
    ExitCode::SUCCESS
}

pub fn handle_dependencies(path: Option<String>) -> ExitCode {
    let root = resolve_target(path);
    println!("Dependency Report — {}", root);
    println!();

    let cargo_lock = std::path::Path::new(&root).join("Cargo.lock");
    if cargo_lock.exists() {
        if let Ok(content) = std::fs::read_to_string(&cargo_lock) {
            let mut direct_count = 0usize;
            let mut transitive_count = 0usize;
            let mut in_package = false;
            let mut pkg_name = String::new();
            let mut pkg_version = String::new();
            let mut packages: Vec<(String, String, &str)> = Vec::new();

            let cargo_toml = std::path::Path::new(&root).join("Cargo.toml");
            let mut direct_deps: std::collections::HashSet<String> =
                std::collections::HashSet::new();
            if let Ok(toml_content) = std::fs::read_to_string(&cargo_toml) {
                let mut in_deps = false;
                for line in toml_content.lines() {
                    if line.trim().starts_with("[dependencies]") {
                        in_deps = true;
                        continue;
                    }
                    if in_deps && line.trim().starts_with('[') {
                        in_deps = false;
                    }
                    if in_deps {
                        if let Some(eq) = line.find('=') {
                            let name = line[..eq].trim().to_string();
                            if !name.is_empty() && !name.starts_with('#') {
                                direct_deps.insert(name);
                            }
                        }
                    }
                }
            }

            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed == "[[package]]" {
                    if !pkg_name.is_empty() && !pkg_version.is_empty() {
                        let dep_type = if direct_deps.contains(&pkg_name) {
                            direct_count += 1;
                            "direct"
                        } else {
                            transitive_count += 1;
                            "transitive"
                        };
                        packages.push((pkg_name.clone(), pkg_version.clone(), dep_type));
                    }
                    pkg_name.clear();
                    pkg_version.clear();
                    in_package = true;
                    continue;
                }
                if in_package {
                    if let Some(name_val) = trimmed.strip_prefix("name = ") {
                        pkg_name = name_val.trim_matches('"').to_string();
                    } else if let Some(ver_val) = trimmed.strip_prefix("version = ") {
                        pkg_version = ver_val.trim_matches('"').to_string();
                    }
                }
            }
            if !pkg_name.is_empty() && !pkg_version.is_empty() {
                let dep_type = if direct_deps.contains(&pkg_name) {
                    direct_count += 1;
                    "direct"
                } else {
                    transitive_count += 1;
                    "transitive"
                };
                packages.push((pkg_name.clone(), pkg_version.clone(), dep_type));
            }

            println!("Language: Rust");
            println!(
                "Dependencies: {} total ({} direct, {} transitive)",
                packages.len(),
                direct_count,
                transitive_count
            );
            println!();
            println!("{:<25} {:<12} Type", "Package", "Version");
            for (name, version, dep_type) in packages.iter().take(30) {
                println!("{:<25} {:<12} {}", name, version, dep_type);
            }
            if packages.len() > 30 {
                println!("... and {} more", packages.len() - 30);
            }
        }
    } else {
        let pyproject = std::path::Path::new(&root).join("pyproject.toml");
        if pyproject.exists() {
            if let Ok(content) = std::fs::read_to_string(&pyproject) {
                println!("Language: Python");
                println!("Dependencies from pyproject.toml:");
                for line in content.lines() {
                    let t = line.trim();
                    if t.contains('=') && !t.starts_with('[') && !t.starts_with('#') {
                        println!("  {}", t);
                    }
                }
            }
        } else {
            let reqs = std::path::Path::new(&root).join("requirements.txt");
            if reqs.exists() {
                if let Ok(content) = std::fs::read_to_string(&reqs) {
                    println!("Language: Python");
                    println!("Dependencies from requirements.txt:");
                    for line in content.lines() {
                        let t = line.trim();
                        if !t.is_empty() && !t.starts_with('#') {
                            println!("  {}", t);
                        }
                    }
                }
            } else {
                println!(
                    "No dependency files found (Cargo.lock, pyproject.toml, requirements.txt)"
                );
            }
        }
    }
    ExitCode::SUCCESS
}
