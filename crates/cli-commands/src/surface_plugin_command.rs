// PURPOSE: PluginCommandsSurface — CLI surface for listing and managing enabled adapters/plugins
use shared::code_analysis::contract_adapter_port::ILinterAdapterPort;
use shared::plugin_system::contract_plugin_commands_aggregate::PluginCommandsAggregate;
use shared::taxonomy_adapter_name_vo::AdapterName;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::process::ExitCode;
use std::sync::Arc;

fn _use_contract_aggregates() {
    let _ = std::marker::PhantomData::<dyn PluginCommandsAggregate>;
}

pub struct PluginCommandsSurface {
    pub linter_adapters: HashMap<String, Arc<dyn ILinterAdapterPort>>,
}

impl PluginCommandsSurface {
    pub fn new(linter_adapters: HashMap<String, Arc<dyn ILinterAdapterPort>>) -> Self {
        Self { linter_adapters }
    }

    pub fn adapters(&self) {
        println!("Enabled Adapters:");
        for name in self.linter_adapters.keys() {
            println!(" - {}", name);
        }
    }

    pub fn plugins(&self) {
        println!("Discovered Plugins:");
        println!("No plugins or custom adapters found.");
        println!("\nTo register a plugin, add entry point in Cargo.toml:");
        println!("  [lib.adapter]");
        println!("  my_adapter = my_package::MyAdapterClass");
    }
}

pub fn handle_adapters(linter_adapters: HashMap<String, Arc<dyn ILinterAdapterPort>>) -> ExitCode {
    let adapter_names = [
        ("clippy", "Rust", "lint, fix"),
        ("ruff", "Python", "lint, fix"),
        ("mypy", "Python", "lint"),
        ("bandit", "Python", "lint"),
        ("eslint", "JavaScript", "lint, fix"),
        ("prettier", "JavaScript", "format"),
        ("tsc", "TypeScript", "lint"),
        ("complexity", "Python", "lint"),
        ("dependency", "Python", "lint"),
    ];

    println!("Available External Adapters");
    println!();
    let mut by_lang: BTreeMap<&str, Vec<(&str, &str)>> = BTreeMap::new();
    for (name, lang, caps) in &adapter_names {
        by_lang.entry(lang).or_default().push((name, caps));
    }
    for (lang, adapters) in &by_lang {
        println!("{}:", lang);
        for (name, caps) in adapters {
            let an = AdapterName::new(name.to_string()).unwrap_or_default();
            let status = if linter_adapters.contains_key(an.value()) {
                "Ready"
            } else {
                "Not found"
            };
            println!("  {:12}  {}  {}", name, status, caps);
        }
        println!();
    }
    ExitCode::SUCCESS
}
