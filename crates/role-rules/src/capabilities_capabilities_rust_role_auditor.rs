// PURPOSE: rust capability role auditor — AES403 sub-checks for Rust files.
//
// Handles the 3-block capability shape as `impl Trait for Type` (Block 2) followed by a bare `impl Type` (Block 3).
//
// The orchestrator (agent_role_orchestrator.rs) selects this auditor by
// `file.language` and calls the trait entry point; the composite entry point
// runs nine AES403 sub-checks — four local helpers (`_block_order`,
// `_constant_placement`, `_test_placement`, `_helper_visibility`) and five
// shared checks in utility_capabilities_role_checker.rs (type budget,
// implementor, single protocol, block markers, banner hygiene).

use shared_common::taxonomy_lint_vo::LintResult;
use shared_common::taxonomy_severity_vo::Severity;
use shared_filesystem::taxonomy_filesystem_vo::{ExternalReferenceMap, FileEntry};
use shared_role_rules::contract_role_protocol::ICapabilitiesRoleProtocol;

use shared_role_rules::utility_capabilities_role_checker;

// ─── Block 1: Module Declaration ───

pub struct CapabilitiesRustRoleAuditor {}
impl CapabilitiesRustRoleAuditor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for CapabilitiesRustRoleAuditor {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Block 2: Module Contract ───
impl ICapabilitiesRoleProtocol for CapabilitiesRustRoleAuditor {
    fn check_capability_routing(
        &self,
        file: &FileEntry,
        layer: &str,
        violations: &mut Vec<LintResult>,
    ) {
        if !utility_capabilities_role_checker::is_capabilities_layer(layer) {
            return;
        }
        let references = ExternalReferenceMap::default();
        self.check_capability_routing_with_references(file, layer, &references, violations);
    }

    fn check_capability_routing_with_references(
        &self,
        file: &FileEntry,
        layer: &str,
        references: &ExternalReferenceMap,
        violations: &mut Vec<LintResult>,
    ) {
        if !utility_capabilities_role_checker::is_capabilities_layer(layer) {
            return;
        }
        utility_capabilities_role_checker::check_type_budget(file, violations);
        utility_capabilities_role_checker::check_implementor(file, violations);
        utility_capabilities_role_checker::check_single_protocol(file, violations);
        self.check_capability_block_order(file, violations);
        utility_capabilities_role_checker::check_block_markers(file, violations);
        self.check_capability_banner_hygiene(file, violations);
        self.check_capability_constant_placement(file, violations);
        self.check_capability_test_placement(file, violations);
        self.check_capability_helper_visibility(file, references, violations);
    }

    fn check_capability_type_budget(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        utility_capabilities_role_checker::check_type_budget(file, violations);
    }

    fn check_capability_implementor(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        utility_capabilities_role_checker::check_implementor(file, violations);
    }

    fn check_capability_single_protocol(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        utility_capabilities_role_checker::check_single_protocol(file, violations);
    }

    fn check_capability_block_order(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        self._block_order(
            &file.content,
            file.path.to_string_lossy().as_ref(),
            violations,
        );
    }

    fn check_capability_block_markers(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        utility_capabilities_role_checker::check_block_markers(file, violations);
    }

    fn check_capability_constructor_block1(
        &self,
        file: &FileEntry,
        violations: &mut Vec<LintResult>,
    ) {
        utility_capabilities_role_checker::check_constructor_in_block1(file, violations);
    }

    fn check_capability_banner_hygiene(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        utility_capabilities_role_checker::check_banner_hygiene(file, violations);
    }

    fn check_capability_block3_pub_facade(
        &self,
        file: &FileEntry,
        violations: &mut Vec<LintResult>,
    ) {
        utility_capabilities_role_checker::check_block3_pub_facade(file, violations);
    }

    fn check_capability_di_default(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        utility_capabilities_role_checker::check_di_default(file, violations);
        utility_capabilities_role_checker::check_impl_default_on_di(file, violations);
    }

    fn check_capability_block2_inline_details(
        &self,
        file: &FileEntry,
        violations: &mut Vec<LintResult>,
    ) {
        utility_capabilities_role_checker::check_block2_inline_details(file, violations);
    }

    fn check_capability_constant_placement(
        &self,
        file: &FileEntry,
        violations: &mut Vec<LintResult>,
    ) {
        self._constant_placement(
            &file.content,
            file.path.to_string_lossy().as_ref(),
            violations,
        );
    }

    fn check_capability_test_placement(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        self._test_placement(
            &file.content,
            file.path.to_string_lossy().as_ref(),
            violations,
        );
    }

    fn check_capability_helper_visibility(
        &self,
        file: &FileEntry,
        references: &ExternalReferenceMap,
        violations: &mut Vec<LintResult>,
    ) {
        self._helper_visibility(
            &file.content,
            file.path.to_string_lossy().as_ref(),
            references,
            violations,
        );
    }
}

// ─── Block 3: Module Helpers ───

impl CapabilitiesRustRoleAuditor {
    // ===============================================================
    // Block order — protocol impl must precede inherent impl
    // ===============================================================

    fn _block_order(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();
        let mut protocol_line: Option<(usize, String)> = None;
        let mut inherent_offenders: Vec<(usize, String)> = Vec::new();
        let mut in_cfg_test = false;

        // Per R2, constructor impls (`impl <Struct> { pub fn new }`,
        // `impl Default for <Struct>`) belong in Block 1, between the
        // `// Block 1:` and `// Block 2:` banners. Only a bare inherent impl
        // that sits at or below the `// Block 3:` banner is a Block 3
        // misplacement. When the file has no banners, a bare inherent impl on
        // a raw line before the protocol impl is a violation. Track banner
        // positions so the comparison is by block, not by raw line number.
        let markers = utility_capabilities_role_checker::capability_block_markers(content);
        let banner1_idx: Option<usize> = markers.iter().find(|m| m.number == 1).map(|m| m.line - 1);
        let banner2_idx: Option<usize> = markers.iter().find(|m| m.number == 2).map(|m| m.line - 1);
        let banner3_idx: Option<usize> = markers.iter().find(|m| m.number == 3).map(|m| m.line - 1);

        for (i, l) in lines.iter().enumerate() {
            let t = l.trim();

            // Skip #[cfg(test)] blocks and everything they contain.
            if t.starts_with("#[cfg(test)]") {
                in_cfg_test = true;
                continue;
            }
            if in_cfg_test {
                if t == "}" || t.starts_with("} //") {
                    in_cfg_test = false;
                }
                continue;
            }

            if protocol_line.is_none()
                && (t.starts_with("impl ") || t.starts_with("impl<"))
                && t.contains("Protocol for")
            {
                // The block-order check uses the looser "Protocol for" substring
                // so it does not miss protocol implementations that use a generic
                // prefix or re-exported trait names.
                protocol_line = Some((i, t.trim_end_matches('{').trim().to_string()));
            }

            if (t.starts_with("impl ") || t.starts_with("impl<")) && !t.contains(" for ") {
                // `impl Default for T` contains ` for ` and is skipped above;
                // bare inherent impls like `impl Type {` have no ` for `.
                // A constructor impl (`impl <Struct> { pub fn new }`) sitting
                // strictly between the Block 1 and Block 2 banners is R2-conformant.
                // When there is a B1 but no B2, anything after B1 is still the
                // constructor region (nothing else can legally occupy it yet).
                let is_constructor_region = match (banner1_idx, banner2_idx) {
                    (Some(b1), Some(b2)) => b1 < i && i < b2,
                    (Some(b1), None) => b1 < i,
                    (None, _) => false,
                };
                if !is_constructor_region {
                    inherent_offenders.push((i, t.trim_end_matches('{').trim().to_string()));
                }
            }
        }

        // Only report when the inherent impl is actually misplaced. For
        // banner-carrying files the violation is an inherent impl before the
        // Block 3 banner (i.e. tucked into Block 1 or 2 instead of Block 3).
        // For banner-less files it is a raw-line ordering violation.
        let proto_src = protocol_line
            .as_ref()
            .map(|(_, s)| s.as_str())
            .unwrap_or("<protocol impl>");
        let proto_idx = protocol_line.as_ref().map(|(i, _)| *i);

        // Report every bare inherent impl that sits outside its legal region:
        // - banner-carrying: an inherent impl above the Block 3 banner is tucked
        //   into Block 1 or Block 2 instead of Block 3 (Rule 1).
        // - banner-less: an inherent impl before the protocol impl is a raw-order
        //   violation (fallback Rule 2).
        for (inh_idx, inh_src) in inherent_offenders {
            let is_misplaced = match banner3_idx {
                Some(b3) => inh_idx < b3,
                None => proto_idx.map_or(false, |p| inh_idx < p),
            };
            if is_misplaced {
                violations.push(LintResult::new_arch_with_name(
                    path,
                    inh_idx + 1,
                    "AES403",
                    Severity::HIGH,
                    format!(
                        "Block 2 (protocol impl) must precede Block 3 (inherent impl): `{inh_src}` is declared before `{proto_src}`.",
                        inh_src = inh_src,
                        proto_src = proto_src,
                    ),
                    "CAPABILITY_ROLE",
                    format!(
                        "`{inh_src}` is declared at line {} but `{proto_src}` follows at line {}.",
                        inh_idx + 1,
                        proto_idx.map(|p| p + 1).unwrap_or(inh_idx + 1)
                    ),
                    match banner3_idx {
                        Some(_) => "Reorder the file so the blocks read:\n  \
                                     Block 1: Module Declaration -> Block 2: Module Contract -> Block 3: Module Helpers.\n  \
                                     Move the bare inherent impl below the `// ─── Block 3: Module Helpers ───` banner.",
                        None => "Reorder the file so the blocks read:\n  \
                                     Block 1: Module Declaration -> Block 2: Module Contract -> Block 3: Module Helpers.\n  \
                                     Move the protocol impl above the inherent impl instead.",
                    },
                ));
            }
        }

        // Protocol placement: the protocol impl must live strictly inside region
        // Block 2 (between the B2 and B3 banners). A protocol impl elsewhere is
        // itself a block-membership defect.
        if let (Some((proto_idx, proto_src)), Some(b2), Some(b3)) =
            (protocol_line, banner2_idx, banner3_idx)
        {
            if !(b2 < proto_idx && proto_idx < b3) {
                violations.push(LintResult::new_arch_with_name(
                    path,
                    proto_idx + 1,
                    "AES403",
                    Severity::HIGH,
                    format!(
                        "Protocol impl must live in region Block 2: `{proto_src}` is outside the `// ─── Block 2: Module Contract ───` .. `// ─── Block 3: Module Helpers ───` span.",
                        proto_src = proto_src,
                    ),
                    "CAPABILITY_ROLE",
                    format!(
                        "`{proto_src}` is declared at line {}, outside region Block 2 (lines {}–{}).",
                        proto_idx + 1,
                        b2 + 1,
                        b3 + 1
                    ),
                    "Move the protocol trait implementation into region Block 2, between \
                     the `// ─── Block 2: Module Contract ───` and `// ─── Block 3: Module Helpers ───` banners.",
                ));
            }
        }
    }

    // ===============================================================
    // Constant placement — policy constants live in `taxonomy_*_constant.*`
    // ===============================================================

    fn _constant_placement(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        // Only file-level constants are in scope. A `const` nested inside an
        // `impl` block is an associated constant — it belongs to the type and
        // cannot be shared, so it is not a placement defect.
        for (i, l) in content.lines().enumerate() {
            let t = l.trim();
            // `const NAME: Type = <literal>;` belongs in taxonomy_<domain>_constant.rs.
            // `static` is excluded: lazily-initialised caches (LazyLock/OnceLock)
            // are implementation detail, not a shareable policy constant.
            let indent = l.len() - l.trim_start().len();
            if indent == 0
                && t.starts_with("const ")
                && !t.starts_with("const fn ")
                && t.contains(':')
                && t.contains('=')
            {
                let name = t
                    .split_whitespace()
                    .nth(1)
                    .unwrap_or("<unnamed>")
                    .trim_end_matches(':')
                    .to_string();
                violations.push(LintResult::new_arch_with_name(
                    path,
                    i + 1,
                    "AES403",
                    Severity::MEDIUM,
                    format!("Local constant in capabilities file: `{name}`."),
                    "CAPABILITY_ROLE",
                    format!(
                        "`{name}` is declared as a file-level `const` at line {}.",
                        i + 1
                    ),
                    format!(
                        "Move `{name}` into `taxonomy_<domain>_constant.rs` so every layer shares one policy value.\n  \
                         Keep the constant in this file only when it is a private mechanical detail, not a domain policy."
                    ),
                ));
            }
        }
    }

    // ===============================================================
    // Test placement — inline tests belong in `tests/`
    // ===============================================================

    fn _test_placement(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();
        for (i, l) in lines.iter().enumerate() {
            let t = l.trim();
            // Only a real declaration counts. A `mod tests` mention inside a
            // comment, a string, or a `starts_with(...)` guard is not one.
            let is_cfg_attr = t == "#[cfg(test)]";
            let is_test_mod = (t.starts_with("mod tests")
                || t.starts_with("pub mod tests")
                || t.starts_with("#[cfg(test)] mod tests"))
                && t.ends_with('{');
            if !is_cfg_attr && !is_test_mod {
                continue;
            }
            if is_cfg_attr {
                // `#[cfg(test)]` on its own line flags the test block that
                // follows; skip intervening `#[allow(…)]`/`#[cfg(…)]` attrs and
                // the `mod tests` on the next line so we report once per block.
                let mut next = i + 1;
                while let Some(n) = lines.get(next) {
                    let nt = n.trim();
                    if nt.starts_with("#[cfg(test)]")
                        || nt.starts_with("#[allow(")
                        || nt.starts_with("#[cfg(")
                    {
                        next += 1;
                        continue;
                    }
                    break;
                }
                let after = lines.get(next).map(|n| n.trim()).unwrap_or("");
                if after.starts_with("mod tests") || after.starts_with("pub mod tests") {
                    continue;
                }
            }
            violations.push(LintResult::new_arch_with_name(
                path,
                i + 1,
                "AES403",
                Severity::LOW,
                "Embedded test code in capabilities file.".to_string(),
                "CAPABILITY_ROLE",
                format!("A test module is declared inline at line {}.", i + 1),
                "Move it into `crates/<crate>/tests/` as a dedicated test file \
                 (for example `tests/unit_capabilities_<name>.rs`) and keep the capability source \
                 free of test-only code.",
            ));
        }
    }

    // ===============================================================
    // Helper visibility — a Block 3 helper is not `pub` without a caller
    // ===============================================================

    fn _helper_visibility(
        &self,
        content: &str,
        path: &str,
        references: &ExternalReferenceMap,
        violations: &mut Vec<LintResult>,
    ) {
        let lines: Vec<&str> = content.lines().collect();
        let mut in_cfg_test_depth: i32 = 0;
        let mut in_inherent_block = false;
        let mut block_depth: i32 = 0;
        let mut in_brace_region: bool = false; // true once we've seen depth > 0 inside the impl

        for (i, l) in lines.iter().enumerate() {
            let t = l.trim();

            // Skip #[cfg(test)] blocks entirely, tracking depth so the
            // closing brace of the whole block (not just the first fn) ends
            // the skip.
            if t.starts_with("#[cfg(test)]") {
                in_cfg_test_depth = 1;
                continue;
            }
            if in_cfg_test_depth > 0 {
                in_cfg_test_depth += i32::from(t.contains('{')) - i32::from(t.contains('}'));
                if in_cfg_test_depth == 0 {
                    continue;
                }
                continue;
            }

            // Track whether we're inside an inherent impl block.
            if t.starts_with("impl ") && !t.contains(" for ") {
                in_inherent_block = true;
                block_depth = 0;
                in_brace_region = false;
            }
            if in_inherent_block {
                block_depth += i32::from(t.contains('{')) - i32::from(t.contains('}'));
                if block_depth > 0 {
                    in_brace_region = true;
                }
                if in_brace_region && block_depth <= 0 {
                    // Closing brace of the impl block; depth returned to 0.
                    in_inherent_block = false;
                    block_depth = 0;
                    in_brace_region = false;
                    continue;
                }
            }
            if !in_inherent_block {
                continue;
            }

            // Constructors and factory methods are exempt.
            if t.starts_with("pub fn ")
                && !t.starts_with("pub fn new(")
                && !t.starts_with("pub fn default(")
                && !t.starts_with("pub fn with_")
            {
                let fn_name = t
                    .split_whitespace()
                    .nth(2)
                    .unwrap_or("")
                    .split('(')
                    .next()
                    .unwrap_or("")
                    .split('<')
                    .next()
                    .unwrap_or("");

                // A method that production code calls -- from another module
                // or from inside this file -- is a genuine public API, not a
                // leak. Skip it.
                if references.referenced_from_production(path, fn_name) {
                    continue;
                }

                // A method called only by integration tests must stay `pub`:
                // `pub(crate)` is invisible from `tests/` targets (they compile
                // as a separate crate). Demoting it would break the test suite.
                if references.referenced_only_from_tests(path, fn_name) {
                    continue;
                }

                // This `pub` helper has zero external callers — it should be
                // private or `pub(crate)`.
                violations.push(LintResult::new_arch_with_name(
                    path,
                    i + 1,
                    "AES403",
                    Severity::MEDIUM,
                    format!("Public helper has no external caller: `{fn_name}`."),
                    "CAPABILITY_ROLE",
                    format!(
                        "`pub fn {}` in {} is not referenced from any other module or test.",
                        fn_name, path
                    ),
                    format!(
                        "Change it to `fn {}` (private). \
                         If a test module in the same crate calls it, change it to `pub(crate) fn {}`.",
                        fn_name, fn_name
                    ),
                ));
            }
        }
    }
}
