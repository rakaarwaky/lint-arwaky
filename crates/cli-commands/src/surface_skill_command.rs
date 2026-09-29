// PURPOSE: Skill read/list — CLI output for embedded AES skill documentation.
// Surface layer: formats EMBEDDED_SKILLS for the terminal, no business logic.
use shared::common::ExitCode;
use shared::project_setup::EmbeddedSkillVO;
use shared::project_setup::taxonomy_project_setup_constant::EMBEDDED_SKILLS;
use std::collections::BTreeMap;

/// One row of the `skill list` table.
struct SkillFileRow {
    relative_path: &'static str,
    language: Option<&'static str>,
}

/// Group every embedded file by its skill name, sorted by name then path.
fn files_by_skill() -> BTreeMap<&'static str, Vec<SkillFileRow>> {
    let mut grouped: BTreeMap<&'static str, Vec<SkillFileRow>> = BTreeMap::new();
    for skill in EMBEDDED_SKILLS {
        grouped.entry(skill.name()).or_default().push(SkillFileRow {
            relative_path: skill.relative_path(),
            language: skill.language(),
        });
    }
    for rows in grouped.values_mut() {
        rows.sort_by_key(|row| row.relative_path);
    }
    grouped
}

/// Look up a skill's main `SKILL.md` entry.
fn main_file(name: &str) -> Option<&'static EmbeddedSkillVO> {
    EMBEDDED_SKILLS
        .iter()
        .find(|skill| skill.name() == name && skill.language().is_none())
}

/// `skill read <name>` — print the skill's `SKILL.md`, and its reference files
/// when `with_references` is set. Unknown names exit 2 per the exit-code contract.
pub fn handle_skill_read(name: &str, with_references: bool) -> ExitCode {
    let Some(main) = main_file(name) else {
        eprintln!("error: skill '{name}' not found");
        eprintln!("run `lint-arwaky skill list` to see available skills");
        return ExitCode::RUNTIME_ERROR;
    };

    if with_references {
        for skill in EMBEDDED_SKILLS
            .iter()
            .filter(|skill| skill.name() == name && skill.language().is_some())
        {
            println!("===== {} =====", skill.relative_path());
            println!("{}", skill.content().trim_end());
            println!();
        }
    }

    println!("===== {} =====", main.relative_path());
    println!("{}", main.content().trim_end());
    ExitCode::OK
}

/// `skill list` — print every skill with its file count and available languages.
pub fn handle_skill_list() -> ExitCode {
    let grouped = files_by_skill();
    let width = grouped.keys().map(|name| name.len()).max().unwrap_or(0);

    for (name, rows) in &grouped {
        let mut languages: Vec<&str> = rows.iter().filter_map(|row| row.language).collect();
        languages.sort_unstable();
        languages.dedup();
        let language_summary = if languages.is_empty() {
            "all".to_string()
        } else {
            languages.join(", ")
        };
        println!(
            "{name:<width$}  {count:>2} file(s)  [{language_summary}]",
            count = rows.len(),
        );
    }
    println!();
    println!("{} skill(s) available.", grouped.len());
    println!("Read one with: lint-arwaky skill read <name>");
    ExitCode::OK
}
