// PURPOSE: LogVerbosity re-export for backward compatibility.
//
// LogVerbosity now lives in `shared_logging`. This file re-exports it so
// that existing imports from `shared_common::taxonomy_logging_vo` continue
// to resolve without adding a second dependency at every call site.
//
// WARNING: shared_common does NOT depend on shared_logging (that would be
// a circular dependency). This file is empty — callers must import from
// `shared_logging::LogVerbosity` directly.

// No re-exports — see header comment.
