pub mod taxonomy;
pub mod contract;
pub mod infrastructure;
pub mod capabilities;
pub mod agent;
pub mod surfaces;
pub mod cli_main_entry;
pub mod mcp_main_entry;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(|s| s.as_str()) == Some("mcp") {
        let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
        rt.block_on(async {
            mcp_main_entry::run_server().await;
        });
    } else {
        cli_main_entry::run_cli();
    }
}
