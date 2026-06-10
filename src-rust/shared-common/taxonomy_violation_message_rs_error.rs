// PURPOSE: AesViolation — AES violation messages for Rust analysis (enum with Display)
use std::fmt;
use crate::shared_common::taxonomy_layer_vo::LayerNameVO;
use crate::shared_common::taxonomy_name_vo::SymbolName;

pub enum AesViolation {
    // AES001 — Import rules
    ForbiddenImport {
        source_layer: LayerNameVO,
        forbidden_layer: LayerNameVO,
        allowed: Vec<LayerNameVO>,
    },
    // AES002 — Mandatory import
    MissingImport {
        source_layer: LayerNameVO,
        required: SymbolName,
    },
    // AES012 — Suffix rules
    SuffixForbidden,
    SuffixMismatch,
    // AES013 — Forbidden inheritance
    ForbiddenInheritance,
    // AES014 — Mandatory inheritance
    MandatoryInheritance,
    // AES015 — Circular import
    CircularImport,
    // AES020 — File size
    FileTooLarge,
    FileTooShort,
    // AES022 — Bypass comments
    BypassComment,
    UnwrapExpect,
    Panic,
    // AES023 — Unused imports
    FixUnusedImport,
    // AES024 — Class/struct definition & dead inheritance
    MandatoryClassDefinition,
    DeadInheritance,
    // AES030 — Orphan code
    OrphanCode,
    // AES0301 — Taxonomy role
    ConstantPurity,
    // AES0302 — Contract primitive
    ContractPrimitive,
    // AES0303 — Capability role
    CapabilityRouting {
        struct_name: SymbolName,
    },
    SingleBottleneck,
    MissingVo,
    // AES0304 — Infrastructure role
    InfrastructureMissingVo,
    // AES0305 — Agent role
    StatelessExecution,
    HighLevelPolicy,
    CoordinatesMultiple,
    NoDomainLogic,
    LazyEagerInit,
    MustImplementContract,
    AnyType,
    // AES0306 — Surface role
    HierarchyViolation,
    PassiveViolation,
    SurfaceRoleViolation,
}

impl fmt::Display for AesViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // AES001
            Self::ForbiddenImport { source_layer, forbidden_layer, allowed } => {
                let allowed_str = if allowed.is_empty() {
                    "none".to_string()
                } else {
                    allowed
                        .iter()
                        .map(|v| v.value().to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                };
                write!(f, "AES001 FORBIDDEN_IMPORT: Layer '{}' is importing from forbidden layer '{}'.\n\
                    WHY? Layer '{}' must not depend on '{}' to maintain architectural boundaries.\n\
                    FIX: Remove the import or refactor to use one of the allowed layers: [{}].",
                    source_layer, forbidden_layer, source_layer, forbidden_layer, allowed_str)
            }
            // AES002
            Self::MissingImport { source_layer, required } =>
                write!(f, "AES002 MANDATORY_IMPORT: Layer '{}' is missing required import '{}'.\n\
                    WHY? Layer '{}' must import '{}' to satisfy architectural contract requirements.\n\
                    FIX: Add the required import statement for '{}' in this file.",
                    source_layer, required, source_layer, required, required),
            // AES012
            Self::SuffixForbidden =>
                write!(f, "AES012 SUFFIX_FORBIDDEN: File uses a forbidden suffix for this layer.\n\
                    WHY? Forbidden suffixes prevent technical concepts from leaking into domain layers.\n\
                    FIX: Rename the file to use an allowed suffix or move it to the correct layer."),
            Self::SuffixMismatch =>
                write!(f, "AES012 SUFFIX_MISMATCH: Contract file missing _port, _protocol, or _aggregate suffix."),
            // AES013
            Self::ForbiddenInheritance =>
                write!(f, "AES013 FORBIDDEN_INHERITANCE: implemented from forbidden source."),
            // AES014
            Self::MandatoryInheritance =>
                write!(f, "AES014 MANDATORY_INHERITANCE: File imports contracts but no class implements them."),
            // AES015
            Self::CircularImport =>
                write!(f, "AES015 CIRCULAR_IMPORT: Circular dependencies detected."),
            // AES020
            Self::FileTooLarge =>
                write!(f, "AES020 FILE_TOO_LARGE: File exceeds the maximum allowed line count.\n\
                    WHY? Large files violate the Single Responsibility Principle.\n\
                    FIX: Split the module into smaller, more focused files"),
            Self::FileTooShort =>
                write!(f, "AES021 FILE_TOO_SHORT: File contains fewer than the required minimum lines.\n\
                    WHY? Excessively small files clutter the project structure.\n\
                    FIX: Expand the component or merge this logic into a related module"),
            // AES022
            Self::BypassComment =>
                write!(f, "AES022 BYPASS_COMMENT: Bypass comment detected."),
            Self::UnwrapExpect =>
                write!(f, "AES022 BYPASS_COMMENT: unwrap/expect call detected."),
            Self::Panic =>
                write!(f, "AES022 BYPASS_COMMENT: panic call detected."),
            // AES023
            Self::FixUnusedImport =>
                write!(f, "AES023 UNUSED_IMPORT: Fixing unused import."),
            // AES024
            Self::MandatoryClassDefinition =>
                write!(f, "AES024 MANDATORY_DEFINITION: File is missing a struct, enum, or trait definition.\n\
                    WHY? Encapsulation in structs/traits is required for proper modularization and contract adherence.\n\
                    FIX: Group functions into a struct or implement a Trait that defines the module interface."),
            Self::DeadInheritance =>
                write!(f, "AES024 DEAD_INHERITANCE: Empty struct or trait detected."),
            // AES030
            Self::OrphanCode =>
                write!(f, "AES030 ORPHAN_CODE: File has no imports, not an entry point."),
            // AES0301
            Self::ConstantPurity =>
                write!(f, "AES0301 TAXONOMY_ROLE: _constant file contains non-constant declaration."),
            // AES0302
            Self::ContractPrimitive =>
                write!(f, "AES0302 CONTRACT_PRIMITIVE: Contract trait/method signature uses primitive types instead of taxonomy VO or constant. WHY? Contracts must enforce VO boundaries. FIX: Replace primitives with VO/constant from taxonomy layer."),
            // AES0303
            Self::CapabilityRouting { struct_name } =>
                write!(f, "AES0303 CAPABILITY_ROLE: Struct '{}' has no trait impl.", struct_name),
            Self::SingleBottleneck =>
                write!(f, "AES0303 CAPABILITY_ROLE: All dispatch routes go to a single capability."),
            Self::MissingVo =>
                write!(f, "AES0303 CAPABILITY_ROLE: Capability method call missing required VO parameter."),
            // AES0304
            Self::InfrastructureMissingVo =>
                write!(f, "AES0304 INFRASTRUCTURE_ROLE: Infrastructure method call missing required VO parameter."),
            // AES0305
            Self::StatelessExecution =>
                write!(f, "Non-stateless behavior detected: state assignment found outside __init__."),
            Self::HighLevelPolicy =>
                write!(f, "Low-level implementation details found (infrastructure import)."),
            Self::CoordinatesMultiple =>
                write!(f, "Orchestrator must manage multiple subsystems."),
            Self::NoDomainLogic =>
                write!(f, "Complex domain logic detected in a passive layer/role."),
            Self::LazyEagerInit =>
                write!(f, "Complex initialization logic found in Container."),
            Self::MustImplementContract =>
                write!(f, "Class must implement ServiceContainerAggregate."),
            Self::AnyType =>
                write!(f, "Any type annotation found in agent orchestrator layer."),
            // AES0306
            Self::HierarchyViolation =>
                write!(f, "AES0306 SURFACE_ROLE: Surface file is not imported from the layer barrel."),
            Self::PassiveViolation =>
                write!(f, "AES0306 SURFACE_ROLE: Surface file contains active domain logic."),
            Self::SurfaceRoleViolation =>
                write!(f, "AES0306 SURFACE_ROLE: Surface file exceeds role mandate."),
        }
    }
}

impl From<AesViolation> for String {
    fn from(v: AesViolation) -> String {
        v.to_string()
    }
}
