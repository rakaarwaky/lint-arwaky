// PURPOSE: ChangedFilesLinter — FR-001 lint half (capabilities layer).
//
// Implements IChangedFilesLintProtocol: run the per-file rule groups
// (quality, role, import, naming) over the files a pre-commit diff detected.
// Workspace-wide groups (orphan, structure, docs) are intentionally excluded —
// they reason about the whole tree, not a diff. Replaces the stub that always
// returned zero violations (issue #582).

use shared_cli_commands::LintResultList;
use shared_common::taxonomy_lint_result_vo::LintResult;
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_paths_vo::FilePathList;
use shared_common::taxonomy_severity_vo::Severity;
use shared_filesystem::FilesystemRequest;
use shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared_filesystem::contract_filesystem_protocol::IParserProtocol;
use shared_filesystem::contract_filesystem_protocol::IWorkspaceProtocol;
use shared_filesystem::taxonomy_filesystem_vo::ImportEntry;
use shared_git_hooks::contract_git_hooks_protocol::IChangedFilesLintProtocol;
use shared_git_hooks::taxonomy_git_hooks_constant::LINTABLE_EXTENSIONS;
use shared_import_rules::IImportRunnerAggregate;
use shared_import_rules::taxonomy_import_rules_request::ImportRequest;
use shared_naming_rules::INamingRunnerAggregate;
use shared_naming_rules::taxonomy_naming_rules_request::NamingRequest;
use shared_quality_rules::CodeAnalysisRequest;
use shared_quality_rules::ICodeAnalysisAggregate;
use shared_role_rules::IRoleRunnerAggregate;
use shared_role_rules::taxonomy_role_rules_request::RoleRequest;

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

/// Marker code for a file that exists but whose content could not be read.
const READ_FAILURE_CODE: &str = "E902";

pub struct ChangedFilesLinter {
    io: Arc<dyn IFileSystemIOProtocol>,
    workspace: Arc<dyn IWorkspaceProtocol>,
    parser: Arc<dyn IParserProtocol>,
    filesystem: Arc<dyn IFilesystemAggregate>,
    quality: Arc<dyn ICodeAnalysisAggregate>,
    role: Arc<dyn IRoleRunnerAggregate>,
    import: Arc<dyn IImportRunnerAggregate>,
    naming: Arc<dyn INamingRunnerAggregate>,
}

impl IChangedFilesLintProtocol for ChangedFilesLinter {
    fn lint_changed_files(&self, files: &FilePathList) -> LintResultList {
        let mut results: Vec<LintResult> = Vec::new();
        let mut entries = Vec::new();

        for fp in &files.values {
            let path = Path::new(fp.value.as_str());
            // Deleted files appear in a diff but no longer exist; skip them.
            if !self.io.path_exists(path) {
                continue;
            }
            if !path
                .extension()
                .and_then(|e| e.to_str())
                .is_some_and(|ext| LINTABLE_EXTENSIONS.contains(&ext))
            {
                continue;
            }
            let content = self
                .filesystem
                .execute(FilesystemRequest::read_lintable_file(
                    &path.to_string_lossy(),
                ))
                .into_content_opt();
            let Some(content) = content else {
                // The file exists but cannot be read: surface it instead of
                // silently passing the hook (issue #582).
                results.push(LintResult::new_arch(
                    fp.value.as_str(),
                    0,
                    READ_FAILURE_CODE,
                    Severity::CRITICAL,
                    format!("failed to read changed file for linting: {}", fp.value),
                ));
                continue;
            };
            let extension = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_string();
            let language = match self
                .workspace
                .detect_language_from_path(path.to_string_lossy().as_ref())
            {
                shared_common::taxonomy_config_language_vo::ConfigLanguage::Rust => {
                    shared_filesystem::taxonomy_filesystem_vo::Language::Rust
                }
                shared_common::taxonomy_config_language_vo::ConfigLanguage::Python => {
                    shared_filesystem::taxonomy_filesystem_vo::Language::Python
                }
                shared_common::taxonomy_config_language_vo::ConfigLanguage::TypeScript => {
                    shared_filesystem::taxonomy_filesystem_vo::Language::TypeScript
                }
            };
            entries.push(shared_filesystem::taxonomy_filesystem_vo::FileEntry {
                path: path.to_path_buf(),
                extension,
                language,
                size: content.len() as u64,
                content,
                parse_ok: true,
                parse_metadata: None,
            });
        }

        if entries.is_empty() {
            return LintResultList::new(results);
        }

        self.parser.parse_all(&mut entries);
        // The changed set carries no cross-file import map: AES201/202/203
        // reason about the file's own imports, which is what a pre-commit
        // gate can soundly enforce.
        let import_map: HashMap<String, Vec<ImportEntry>> = HashMap::new();

        results.extend(
            self.quality
                .execute(CodeAnalysisRequest::run_analysis(&entries))
                .into_violations(),
        );
        results.extend(
            self.role
                .execute(RoleRequest::audit(&entries))
                .into_violations(),
        );
        results.extend(
            self.import
                .execute(ImportRequest::audit_with_entries_and_imports(
                    &entries,
                    &import_map,
                ))
                .into_violations(),
        );
        results.extend(
            self.naming
                .execute(NamingRequest::audit(&entries))
                .into_violations(),
        );

        LintResultList::new(results)
    }
}

impl ChangedFilesLinter {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        io: Arc<dyn IFileSystemIOProtocol>,
        workspace: Arc<dyn IWorkspaceProtocol>,
        parser: Arc<dyn IParserProtocol>,
        filesystem: Arc<dyn IFilesystemAggregate>,
        quality: Arc<dyn ICodeAnalysisAggregate>,
        role: Arc<dyn IRoleRunnerAggregate>,
        import: Arc<dyn IImportRunnerAggregate>,
        naming: Arc<dyn INamingRunnerAggregate>,
    ) -> Self {
        Self {
            io,
            workspace,
            parser,
            filesystem,
            quality,
            role,
            import,
            naming,
        }
    }
}
