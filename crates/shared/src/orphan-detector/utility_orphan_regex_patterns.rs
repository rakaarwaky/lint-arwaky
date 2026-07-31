// PURPOSE: DEPRECATED — regex patterns replaced by AST parsing in utility_orphan_ast_parser.
// Kept temporarily for backward compatibility. Will be removed in v2.0.
//
// All functions in this module are no longer used by the AST-based graph resolver.
// They remain only to avoid breaking any external consumers during migration.

#![deprecated(
    since = "1.12.0",
    note = "Use utility_orphan_ast_parser instead. All regex-based parsing has been replaced by AST."
)]

// Intentionally empty. All regex patterns have been removed.
// The AST parser (utility_orphan_ast_parser) handles all parsing needs.
