// PURPOSE: AesViolationPy — AES violation messages for Python analysis (enum with Display)
use std::fmt;

pub enum AesViolationPy {
    // AES001 — Import rules
    ForbiddenImport,
    // AES002 — Mandatory import
    MissingImport {
        source_layer: String,
        required: String,
    },
    // AES012 — Suffix rules
    SuffixForbidden,
    // AES013 — Forbidden inheritance
    ForbiddenInheritance,
    // AES020 — File size
    FileTooLarge,
    FileTooShort,
    // AES024 — Class definition
    MandatoryClassDefinition,
    // AES0301 — Taxonomy role
    ConstantPurity,
    // AES0302 — Contract primitive
    ContractPrimitive,
    // AES0305 — Agent role
    StatelessExecution,
    HighLevelPolicy,
    CoordinatesMultiple,
    NoDomainLogic,
    LazyEagerInit,
    MustImplementContract,
    AnyType,
}

impl fmt::Display for AesViolationPy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // AES001
            Self::ForbiddenImport =>
                write!(f, "AES001 FORBIDDEN_IMPORT: Layer is importing from a forbidden module."),
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
            // AES013
            Self::ForbiddenInheritance =>
                write!(f, "AES013 FORBIDDEN_INHERITANCE: implemented from forbidden source."),
            // AES020
            Self::FileTooLarge =>
                write!(f, "AES020 FILE_TOO_LARGE: File exceeds the maximum allowed line count.\n\
                    WHY? Large files violate the Single Responsibility Principle.\n\
                    FIX: Split the module into smaller, more focused files"),
            Self::FileTooShort =>
                write!(f, "AES021 FILE_TOO_SHORT: File contains fewer than the required minimum lines.\n\
                    WHY? Excessively small files clutter the project structure.\n\
                    FIX: Expand the component or merge this logic into a related module"),
            // AES024
            Self::MandatoryClassDefinition =>
                write!(f, "AES024 MANDATORY_DEFINITION: File is missing a class, dataclass, or Protocol definition.\n\
                    WHY? Encapsulation in classes/Protocols is required for proper modularization and contract adherence.\n\
                    FIX: Group functions into a class or implement a Protocol that defines the module interface."),
            // AES0301
            Self::ConstantPurity =>
                write!(f, "AES0301 CONSTANT_PURITY: _constant file contains non-constant declaration."),
            // AES0302
            Self::ContractPrimitive =>
                write!(f, "AES0302 CONTRACT_PRIMITIVE: Contract trait/method signature uses primitive types instead of taxonomy VO or constant. WHY? Contracts must enforce VO boundaries. FIX: Replace primitives with VO/constant from taxonomy layer."),
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
        }
    }
}

impl From<AesViolationPy> for String {
    fn from(v: AesViolationPy) -> String {
        v.to_string()
    }
}
