// PURPOSE: ImportError — domain-level error type for the import analysis domain.
//
// Represents error conditions that arise during import rule enforcement (AES201-205).
// These are **domain errors**, not infrastructure errors — they capture semantic
// failures such as unresolvable modules, missing symbols in barrel files, and
// circular dependency cycles.
//
// As a taxonomy Error, this type uses only Value Objects / Constants for its
// fields (no raw primitives) and depends only on the taxonomy layer.
//
// # Usage
// ```rust,ignore
// fn check_imports(&self) -> Result<LintResultList, ImportError> {
//     let module = resolve_module(path)?;     // could return ModuleResolution
//     let symbol = resolve_symbol(module)?;   // could return SymbolNotFound
//     Ok(results)
// }
// ```

use shared_common::taxonomy_common_vo::ErrorMessage;
use shared_common::taxonomy_layer_vo::LayerNameVO;
use shared_common::taxonomy_name_vo::SymbolName;
use shared_common::taxonomy_path_vo::FilePath;
use std::fmt;

/// Domain-level errors for import analysis.
///
/// Each variant captures a specific category of import-domain failure
/// using taxonomy Value Objects (no raw `String`, no primitives).
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum ImportError {
    /// A module referenced in an import statement could not be resolved
    /// to any known module file.
    ModuleResolution {
        /// The module path that failed to resolve (e.g. `"modules.shared.src.server"`).
        module: SymbolName,
        /// Human-readable explanation of why resolution failed.
        reason: Option<ErrorMessage>,
    },

    /// An imported symbol was not found in the barrel file's re-exports.
    SymbolNotFound {
        /// The module path (barrel directory) that was searched.
        module: SymbolName,
        /// The symbol that was expected but not found.
        symbol: SymbolName,
    },

    /// A circular dependency was detected between architectural layers.
    CircularDependency {
        /// Edge key describing the cycle (e.g. `"contract(protocol)->capabilities"`).
        cycle: SymbolName,
        /// Source file involved in the cycle.
        file: Option<FilePath>,
    },

    /// Import configuration is invalid or incomplete.
    InvalidConfiguration {
        /// The architectural layer affected by the configuration issue.
        layer: LayerNameVO,
        /// Description of what is wrong with the configuration.
        message: ErrorMessage,
    },
}

impl fmt::Display for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImportError::ModuleResolution { module, reason } => match reason {
                Some(r) => write!(f, "Module '{}' cannot be resolved: {}", module, r),
                None => write!(f, "Module '{}' cannot be resolved", module),
            },
            ImportError::SymbolNotFound { module, symbol } => {
                write!(f, "Symbol '{}' not found in module '{}'", symbol, module)
            }
            ImportError::CircularDependency { cycle, file } => match file {
                Some(path) => write!(f, "Circular dependency '{}' at '{}'", cycle, path),
                None => write!(f, "Circular dependency: {}", cycle),
            },
            ImportError::InvalidConfiguration { layer, message } => {
                write!(
                    f,
                    "Invalid import configuration for layer '{}': {}",
                    layer, message
                )
            }
        }
    }
}

impl ImportError {
    /// Stable numeric id for machine branching.
    pub fn error_id(&self) -> u16 {
        match self {
            Self::ModuleResolution { .. } => 4001,
            Self::SymbolNotFound { .. } => 4002,
            Self::CircularDependency { .. } => 4003,
            Self::InvalidConfiguration { .. } => 4004,
        }
    }

    /// Stable machine-readable name for error classification.
    pub fn error_code(&self) -> &'static str {
        match self {
            Self::ModuleResolution { .. } => "IMPORT_MODULE_UNRESOLVED",
            Self::SymbolNotFound { .. } => "IMPORT_SYMBOL_NOT_FOUND",
            Self::CircularDependency { .. } => "IMPORT_CIRCULAR_DEPENDENCY",
            Self::InvalidConfiguration { .. } => "IMPORT_INVALID_CONFIG",
        }
    }

    /// Human-readable description derived from the error id, code, and VOs.
    pub fn message(&self) -> String {
        format!("{} {}: {}", self.error_id(), self.error_code(), self)
    }
}
