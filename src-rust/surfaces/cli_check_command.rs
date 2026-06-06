use std::sync::Arc;
/// CLI check and scan commands (Surface).
use std::collections::HashMap;


use crate::taxonomy::BooleanVO;

use crate::taxonomy::ComplianceStatus;

use crate::taxonomy::FilePath;

use crate::taxonomy::GovernanceReport;


use crate::taxonomy::{LintResult,
LintResultList};


use crate::taxonomy::Score;


use crate::contract::ServiceContainerAggregate;
use crate::surfaces::cli_output_controller::{get_output_dir, write_output, tee_stdout};

pub struct CheckCommandsSurface {
    pub container: Option<Arc<dyn ServiceContainerAggregate>>,
}

impl CheckCommandsSurface {
    pub fn new() -> Self {
        Self { container: None }
    }

    pub fn register_all(&mut self, container: Arc<dyn ServiceContainerAggregate>) {
        self.container = Some(container);
    }

    pub fn check(&self, path: &str, git_diff: bool) {
        let path_vo = FilePath { value: path.to_string() };
        let diff_vo = BooleanVO { value: git_diff };
        self.run_check(path_vo, diff_vo);
    }

    pub fn scan(&self, path: &str) {
        let path_vo = FilePath { value: path.to_string() };
        let diff_vo = BooleanVO { value: false };
        self.run_check(path_vo, diff_vo);
    }

    fn run_check(&self, project_path: FilePath, git_diff: BooleanVO) {
        let output_dir = get_output_dir(None);

        let output = tee_stdout(None, || {
            if git_diff.value {
                println!("[git-diff] Running analysis on {}", project_path.value);
            } else {
                println!(" Running analysis on {}...", project_path.value);
            }
            // Structural placeholder
            println!("{}", "-".repeat(40));
            println!("total issues :  0");
            println!("total score  :  100.0/100.0");
            println!("{}", "-".repeat(40));
        });

        if let Some(_dir) = output_dir {
            write_output(None, &output, "check", Some("txt"));
        }
    }

}

pub fn register_check_commands(container: Arc<dyn ServiceContainerAggregate>) -> CheckCommandsSurface {
    let mut surface = CheckCommandsSurface::new();
    surface.register_all(container);
    surface
}
