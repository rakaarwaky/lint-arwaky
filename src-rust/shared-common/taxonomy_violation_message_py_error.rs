// PURPOSE: AesViolationPy — AES violation messages for Python analysis (enum with Display)
use std::fmt;

pub enum AesViolationPy {
    ForbiddenImport,
    MissingImport(String),
    FileTooLarge,
    FileTooShort,
    MandatoryClassDefinition,
    SuffixForbidden,
    StatelessExecution,
    HighLevelPolicy,
    CoordinatesMultiple,
    NoDomainLogic,
    LazyEagerInit,
    MustImplementContract,
    AnyType,
    ForbiddenInheritance,
    ConstantPurity,
    ContractPrimitive,
    OrchestratorCaller,
}

impl fmt::Display for AesViolationPy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ForbiddenImport =>
                write!(f, "AES001 FORBIDDEN_IMPORT: Layer is importing from a forbidden module."),
            Self::MissingImport(required) =>
                write!(f, "AES002 MANDATORY_IMPORT: Missing required import: '{}'.", required),
            Self::FileTooLarge =>
                write!(f, "AES020 FILE_TOO_LARGE: File exceeds the maximum allowed line count.\n\
                    WHY? Large files violate the Single Responsibility Principle.\n\
                    FIX: Split the module into smaller, more focused files"),
            Self::FileTooShort =>
                write!(f, "AES021 FILE_TOO_SHORT: File contains fewer than the required minimum lines.\n\
                    WHY? Excessively small files clutter the project structure.\n\
                    FIX: Expand the component or merge this logic into a related module"),
            Self::MandatoryClassDefinition =>
                write!(f, "AES024 MANDATORY_DEFINITION: File is missing a class, dataclass, or Protocol definition.\n\
                    WHY? Encapsulation in classes/Protocols is required for proper modularization and contract adherence.\n\
                    FIX: Group functions into a class or implement a Protocol that defines the module interface."),
            Self::SuffixForbidden =>
                write!(f, "AES012 SUFFIX_FORBIDDEN: File uses a forbidden suffix for this layer.\n\
                    WHY? Forbidden suffixes prevent technical concepts from leaking into domain layers.\n\
                    FIX: Rename the file to use an allowed suffix or move it to the correct layer."),
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
            Self::ForbiddenInheritance =>
                write!(f, "AES013 FORBIDDEN_INHERITANCE: implemented from forbidden source."),
            Self::ConstantPurity =>
                write!(f, "AES0301 CONSTANT_PURITY: _constant file contains non-constant declaration."),
            Self::ContractPrimitive =>
                write!(f, "AES0302 CONTRACT_PRIMITIVE: Contract trait/method signature uses primitive types instead of taxonomy VO or constant. WHY? Contracts must enforce VO boundaries. FIX: Replace primitives with VO/constant from taxonomy layer."),
            Self::OrchestratorCaller =>
                write!(f, "AES0307 ORCHESTRATOR_CALLER: Contract port/protocol not called by any orchestrator (agent_*_orchestrator). WHY? Orchestrator is the primary caller of contracts. FIX: Wire the contract into an orchestrator file."),
        }
    }
}
