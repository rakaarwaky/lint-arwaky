// PURPOSE: NamingConventionChecker — AES101 stem-shape naming validation capability
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;
use shared_common::taxonomy_layer_vo::LayerNameVO;
use shared_common::taxonomy_layer_vo::{LayerDefinition, LayerMapVO};
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_path_vo::FilePathList;
use shared_common::taxonomy_severity_vo::Severity;
use shared_common::taxonomy_violation_item_vo::ViolationItem;
use shared_config_system::taxonomy_config_system_vo::ArchitectureConfig;
use shared_naming_rules::contract_naming_checker_protocol::INamingConventionProtocol;
use shared_naming_rules::taxonomy_naming_rules_constant::{
    AES101_FOLDER_RULE, AES101_SHAPE_RULE, LAYER_PREFIXES, MIN_WORDS_DEFAULT,
    RULE_CODE_NAMING_CONVENTION,
};
use shared_naming_rules::utility_naming_checker::{
    basename_of, detect_layer, get_stem, parse_path, rule_exception_set, string_filename_violation,
};
use shared_naming_rules::utility_naming_output::{exemption_clause, naming_finding};

use std::sync::OnceLock;

// ─── Block 1: Module Declaration ──────────────────────────────

/// Stateless AES101 naming-convention checker (stem shape validation).
///
/// Checks that each file's stem is a sequence of lowercase words joined by
/// underscores, with a configurable minimum word count, and that the stem
/// starts with the prefix of the folder the file sits in. No internal state.
pub struct NamingConventionChecker {}

/// The one stem rule a stem breaks.
///
/// A stem that breaks two rules produces one finding, naming the first: the
/// reader fixes that one, re-runs, and meets the next. A finding that says
/// "may have uppercase characters, a wrong separator, or too few words" makes
/// the reader test three guesses by hand.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BrokenRule {
    /// The stem carries an uppercase letter.
    Casing,
    /// The stem carries a character a name may not hold, or an empty word.
    Separator,
    /// The stem holds fewer words than the configured minimum.
    WordCount,
}

impl Default for NamingConventionChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl NamingConventionChecker {
    pub fn new() -> Self {
        Self {}
    }
}

// ─── Block 2: Module Contract ────────────────

impl INamingConventionProtocol for NamingConventionChecker {
    /// FR-NamingRules-001 — AES101: check each file's stem against the
    /// layer_concern_role convention.
    fn check_file_naming(
        &self,
        config: &ArchitectureConfig,
        layer_map: &LayerMapVO,
        files: &FilePathList,
        _root_dir: &FilePath,
        results: &mut Vec<ViolationItem>,
    ) {
        let layer_keys: Vec<String> = layer_map.values.keys().map(|k| k.to_string()).collect();
        let min_words = Self::min_words_from_config(config);
        let exceptions = rule_exception_set(config, RULE_CODE_NAMING_CONVENTION);

        let violations: Vec<ViolationItem> = files
            .values
            .par_iter()
            .filter_map(|f| {
                let f_str = f.to_string();
                let filename = basename_of(&f_str);
                // Rule-level exceptions evaluated before layer detection (FRD FR-001).
                if exceptions.iter().any(|v| v == filename) {
                    return None;
                }
                let layer = detect_layer(&f_str, &layer_keys);
                let layer_name = layer.as_ref().map(|l| LayerNameVO::new(l.clone()));
                let def = layer_name.as_ref().and_then(|l| layer_map.values.get(l));
                self.check_file_naming_internal(&f_str, filename, &layer_name, def, min_words)
            })
            .collect();

        results.extend(violations);
    }
}

// ─── Block 3: Module Helpers ────────────

impl NamingConventionChecker {
    fn min_words_from_config(config: &ArchitectureConfig) -> usize {
        let value = config.naming.word_count.value;
        if value <= 0 {
            return MIN_WORDS_DEFAULT;
        }
        usize::try_from(value).unwrap_or(MIN_WORDS_DEFAULT)
    }

    /// Regex cache: word counts 1..=10 use the cached `REGEX_TABLE` slots so
    /// repeated checks don't rebuild the pattern. Counts above the table
    /// (min_words >= 11) build the pattern on demand with an unbounded tail
    /// (`{N-1,}`), so the configured value is honored exactly — a 10-word
    /// stem is rejected when the configured minimum is 11, 15, or 100.
    fn naming_regex(min_words: usize) -> Option<Regex> {
        static REGEX_TABLE: [OnceLock<Option<Regex>>; 10] = [
            OnceLock::new(),
            OnceLock::new(),
            OnceLock::new(),
            OnceLock::new(),
            OnceLock::new(),
            OnceLock::new(),
            OnceLock::new(),
            OnceLock::new(),
            OnceLock::new(),
            OnceLock::new(),
        ];

        fn build(min_words: usize) -> Option<Regex> {
            let pattern = format!(
                r"^[a-z0-9]+(_[a-z0-9]+){{{},}}$",
                min_words.saturating_sub(1)
            );
            Regex::new(&pattern).ok()
        }

        let effective = min_words.max(1);
        if effective <= REGEX_TABLE.len() {
            REGEX_TABLE[effective - 1]
                .get_or_init(|| build(effective))
                .as_ref()
                .cloned()
        } else {
            build(effective)
        }
    }

    /// Check file naming conventions (AES101: stem shape — lowercase words,
    /// underscore separator, minimum word count, known folder prefix).
    ///
    /// B-6a test seam: `pub` only so cross-crate unit tests can drive the
    /// per-file rule without building a `FilePathList`; production traffic
    /// enters through [`INamingConventionProtocol::check_file_naming`].
    pub fn check_file_naming_internal(
        &self,
        file: &str,
        filename: &str,
        layer_name: &Option<LayerNameVO>,
        definition: Option<&LayerDefinition>,
        min_words: usize,
    ) -> Option<ViolationItem> {
        let fp = parse_path(filename)?;
        if fp.is_barrel_file() || fp.is_entry_point() {
            return None;
        }

        let stem = get_stem(filename)?;

        if let Some(def) = definition
            && def.exceptions.values.iter().any(|v| v == filename)
        {
            return None;
        }

        // Harden AES101: unprefixed files (no recognized layer prefix) cannot
        // slip through. Every production source file must carry a recognized
        // layer prefix, because the prefix is what places the file in the
        // architecture.
        let folder = layer_name.as_ref().map(|l| base_folder_of(l.value()));
        if folder.is_none() {
            return Some(Self::no_prefix_violation(file, filename, stem));
        }

        if !Self::naming_regex(min_words).is_some_and(|re| re.is_match(stem)) {
            let broken = Self::first_broken_rule(stem);
            return Some(Self::shape_violation(
                file,
                filename,
                stem,
                broken,
                min_words,
                folder.as_deref(),
            ));
        }

        None
    }

    /// The first shape rule the stem breaks.
    ///
    /// The rules are met in the order a reader meets them: the casing, then
    /// the separator, then the word count. The three cover the whole of the
    /// pattern the gate applies, so the word count is the rule that catches a
    /// stem the other two accepted.
    fn first_broken_rule(stem: &str) -> BrokenRule {
        if stem.chars().any(|c| c.is_ascii_uppercase()) {
            return BrokenRule::Casing;
        }
        if stem.chars().any(|c| !c.is_ascii_alphanumeric() && c != '_')
            || stem.split('_').any(|word| word.is_empty())
        {
            return BrokenRule::Separator;
        }
        BrokenRule::WordCount
    }

    /// The finding for a stem that names no known folder.
    ///
    /// This is the one AES101 finding about placement rather than spelling:
    /// without the prefix the file cannot be placed in the architecture at
    /// all, so it outranks a stem that merely breaks the shape rules.
    fn no_prefix_violation(file: &str, filename: &str, stem: &str) -> ViolationItem {
        let problem = format!("Stem '{stem}' does not start with a known folder prefix.");
        let example = "capabilities_user_checker.rs";
        let action = format!(
            "Rename the file so its name starts with a legal prefix, for example '{example}'."
        );
        let legal = format!("Legal prefixes: {}.", LAYER_PREFIXES.join(", "));
        let exemption = exemption_clause(RULE_CODE_NAMING_CONVENTION, filename);
        let finding = naming_finding(
            &[problem.as_str()],
            &[
                action.as_str(),
                legal.as_str(),
                "Then update the imports that name it.",
                "Re-run the check to confirm the finding is gone.",
                exemption.as_str(),
            ],
            &[AES101_FOLDER_RULE],
        );
        string_filename_violation(
            file,
            RULE_CODE_NAMING_CONVENTION,
            finding.message,
            Severity::HIGH,
            "NAMING_CONVENTION",
            finding.why,
            finding.fix,
        )
    }

    /// The finding for a stem that breaks one shape rule.
    ///
    /// The problem clause names the one rule that broke, the fix names the
    /// file to rename to, and the two clauses after it close the loop: the
    /// references move with the file, and the next run confirms it.
    fn shape_violation(
        file: &str,
        filename: &str,
        stem: &str,
        broken: BrokenRule,
        min_words: usize,
        folder: Option<&str>,
    ) -> ViolationItem {
        let problem = match broken {
            BrokenRule::Casing => format!("Stem '{stem}' contains uppercase characters."),
            BrokenRule::Separator => {
                format!("Stem '{stem}' {}.", Self::separator_detail(stem))
            }
            BrokenRule::WordCount => format!(
                "Stem '{stem}' has {} words; the minimum is {min_words}.",
                Self::word_count(stem)
            ),
        };
        // The action names one concrete target: the repaired stem when the
        // repair reaches the configured word count, and otherwise the worked
        // example, which carries the same words and is legal too.
        let target = Self::suggested_file_name(stem, filename, min_words)
            .unwrap_or_else(|| Self::example_file_name(filename, folder, stem, min_words));
        let action = format!("Rename the file to '{target}'.");
        let exemption = exemption_clause(RULE_CODE_NAMING_CONVENTION, filename);
        let finding = naming_finding(
            &[problem.as_str()],
            &[
                action.as_str(),
                "Then update the imports that name it.",
                "Re-run the check to confirm the finding is gone.",
                exemption.as_str(),
            ],
            &[AES101_SHAPE_RULE],
        );
        string_filename_violation(
            file,
            RULE_CODE_NAMING_CONVENTION,
            finding.message,
            Severity::MEDIUM,
            "NAMING_CONVENTION",
            finding.why,
            finding.fix,
        )
    }

    /// What is wrong with the stem's separators, in one clause.
    fn separator_detail(stem: &str) -> String {
        if let Some(found) = stem
            .chars()
            .find(|c| !c.is_ascii_alphanumeric() && *c != '_')
        {
            return format!("uses '{found}', which a file name may not hold");
        }
        if stem.starts_with('_') {
            return "starts with an underscore".to_string();
        }
        if stem.ends_with('_') {
            return "ends with an underscore".to_string();
        }
        "has an empty word between two underscores".to_string()
    }

    /// The number of words the stem holds.
    fn word_count(stem: &str) -> usize {
        stem.split('_').count()
    }

    /// A worked example file name for one broken stem.
    ///
    /// The example carries the words the file already holds, lowercased, and
    /// it carries the folder the file sits in, so the name is legal in every
    /// respect the rules check. Plain role words fill the gap to the
    /// configured minimum; the words the file already carries are never
    /// repeated. The reader can copy the example as it stands.
    fn example_file_name(
        filename: &str,
        folder: Option<&str>,
        stem: &str,
        min_words: usize,
    ) -> String {
        const FILLERS: [&str; 6] = [
            "user", "checker", "service", "handler", "manager", "adapter",
        ];
        let mut words = Self::split_words(stem);
        // The folder word leads the name, the way the convention reads it.
        if let Some(folder) = folder
            && words.first().map(String::as_str) != Some(folder)
        {
            words.insert(0, folder.to_string());
        }
        // One plain role word per missing word, skipping the words the file
        // already carries so the example never repeats itself.
        for filler in FILLERS {
            if words.len() >= min_words {
                break;
            }
            if !words.iter().any(|word| word == filler) {
                words.push(filler.to_string());
            }
        }
        // A word count beyond the filler list still needs words; the number
        // keeps every one of them distinct.
        let mut number = 2;
        while words.len() < min_words {
            words.push(format!("part{number}"));
            number += 1;
        }
        let example = words.join("_");
        match std::path::Path::new(filename)
            .extension()
            .and_then(|e| e.to_str())
        {
            Some(extension) => format!("{example}.{extension}"),
            None => example,
        }
    }

    /// The words a stem holds, lowercased.
    ///
    /// A capital letter starts a new word, and every character a name may not
    /// hold separates two words.
    fn split_words(stem: &str) -> Vec<String> {
        let mut words: Vec<String> = Vec::new();
        let mut current = String::new();
        let mut previous_was_lower = false;
        for character in stem.chars() {
            if character.is_ascii_uppercase() {
                if previous_was_lower && !current.is_empty() {
                    words.push(std::mem::take(&mut current));
                }
                current.push(character.to_ascii_lowercase());
                previous_was_lower = false;
            } else if character.is_ascii_alphanumeric() {
                current.push(character.to_ascii_lowercase());
                previous_was_lower = character.is_ascii_lowercase() || character.is_ascii_digit();
            } else {
                if !current.is_empty() {
                    words.push(std::mem::take(&mut current));
                }
                previous_was_lower = false;
            }
        }
        if !current.is_empty() {
            words.push(current);
        }
        words
    }

    /// The file name the reader should rename the file to.
    ///
    /// `None` when the repair cannot reach the configured word count: a
    /// missing concept cannot be invented, so the finding then names the
    /// pattern instead of a target that would still be wrong.
    fn suggested_file_name(stem: &str, filename: &str, min_words: usize) -> Option<String> {
        let repaired = Self::suggested_stem(stem, min_words)?;
        match std::path::Path::new(filename)
            .extension()
            .and_then(|e| e.to_str())
        {
            Some(extension) => Some(format!("{repaired}.{extension}")),
            None => Some(repaired),
        }
    }

    /// The closest legal stem for one that breaks the shape rules.
    ///
    /// Lowercase letters, digits and underscores are kept; a capital letter
    /// starts a new word, and any other character becomes an underscore.
    /// `None` when the repair cannot reach the configured word count: a
    /// missing word cannot be invented, so the finding then offers an
    /// example instead of a target that would still be wrong.
    fn suggested_stem(stem: &str, min_words: usize) -> Option<String> {
        let words = Self::split_words(stem);
        if words.len() < min_words {
            return None;
        }
        Some(words.join("_"))
    }
}

/// The base folder name of a layer display string.
///
/// A specialized layer reads `capabilities(command)`; the folder a file name
/// may carry is the base one.
fn base_folder_of(layer_display: &str) -> String {
    layer_display
        .split('(')
        .next()
        .unwrap_or(layer_display)
        .to_string()
}
