pub const PYTHON_CONFIG_TEMPLATE: &str = r#"
thresholds:
  score: 100.0
  complexity: 15

output_dir: "output"

adapters:
  - name: "ruff"
    weight: 1.0
  - name: "mypy"
    weight: 1.0
  - name: "bandit"
    weight: 1.0
  - name: "radon"
    weight: 1.0
  - name: "architecture"
    weight: 3.0

ignored_rules: []

ignored_paths:
  - "./output"
  - "/test project"
  - "/.git"
  - "/.github"
  - "/.venv"
  - "/.vscode"
  - "/__pycache__"
  - "*.pyc"
  - ".ipynb_checkpoints"
  - "/dist"
  - "/build"
  - "/scratch"
  - "/test-project-python"
  - "test-project-javascript"

architecture:
  enabled: true
  layers:
    root: 
      path: "src-python/"
      recursive: false
      suffix:
        - strict: ["entry"]
    taxonomy:
      path: "src-python/taxonomy"
      recursive: true
      suffix:
        - strict: ["vo", "entity", "error", "event", "bridge"]
    contract:
      recursive: true
      path: "src-python/contract"
      suffix:
        - strict: ["port", "protocol", "aggregate"]
    capabilities:
      recursive: true
      path: "src-python/capabilities"
      suffix:
        - flexible:
            [
              "analyzer", "actions", "formatters", "generator", "processor",
              "evaluator", "checker", "validator", "transformer", "calculator",
              "builder", "handler", "executor", "resolver", "compiler",
              "aggregator", "classifier", "extractor", "reporter", "mapper",
              "filter", "collector", "comparator", "scorer", "inspector",
              "reviewer", "assessor",
            ]
        - forbidden: ["vo", "entity", "error", "event", "port", "protocol", "io"]
    infrastructure:
      recursive: true
      path: "src-python/infrastructure"
      suffix:
        - flexible:
            [
              "adapter", "provider", "scanner", "client", "tracer", "tracker",
              "variants", "detector", "patterns", "util", "system", "repository",
              "cache", "store", "loader", "writer", "reader", "driver",
              "connector", "gateway", "serializer", "encoder", "decoder",
              "fetcher", "watcher", "indexer", "dispatcher", "recorder",
              "proxy", "publisher", "subscriber", "listener", "poller", "streamer",
            ]
        - forbidden: ["vo", "entity", "error", "event", "port", "protocol", "io"]
    surfaces:
      recursive: true
      path: "src-python/surfaces"
      suffix:
        - strict: [
            "page", "command", "handler", "controller", "router",
            "component", "layout", "view", "entry",
            "hook", "store", "provider"
          ]
       
    agent:
      path: "src-python/agent"
      recursive: true
      suffix:
        - strict:
            [
              "container", "manager", "orchestrator", "registry", "coordinator"
            ]

  rules:
    global:
      - name: "Forbid Any Type"
        forbid_any_type: true
        exceptions: ["rust_taxonomy_bridge.py", "js_taxonomy_bridge.py"]
        forbid_any_type_violation_message: >
          AES024 ANY_TYPE_BYPASS: `Any` type detected in code.
          WHY? Using `Any` bypasses type safety and conflicts with taxonomy Value Objects which require explicit types.
          FIX: Replace `Any` with specific taxonomy types (_vo, _entity), protocols, or `object` if truly generic.
      - name: "Naming Convention"
        word_count: 3
        exceptions: ["__init__.py"]
        word_count_violation_message: >
          AES003 NAMING_CONVENTION: Filename does not follow the word1_word2_word3.py pattern.
          WHY? Strict three-word names ensure architectural consistency and prevent naming ambiguity.
          FIX: Rename the file to exactly three words separated by underscores (e.g., user_data_vo.py).

      - name: "forbidden bypass / cheating"
        forbidden_bypass: ["# noqa", "# type: ignore"]
        forbidden_bypass_violation_message: >
          AES014 BYPASS_COMMENT_VIOLATION: Forbidden bypass comments (noqa, type: ignore) detected.
          WHY? Suppressing linter or type errors masks underlying design flaws and technical debt.
          FIX: Resolve the actual code violation or type mismatch instead of hiding it.
        forbidden_bypass_custom_messages:
          - pattern: "noqa"
            message: >
              AES014 BYPASS_COMMENT_VIOLATION: NOQA comment detected in source code.
              WHY? Linter errors must be resolved at the source to maintain code quality standards.
              FIX: Fix the reported code violation instead of using a suppression comment.
          - pattern: "type: ignore"
            message: >
              AES014 BYPASS_COMMENT_VIOLATION: type: ignore comment detected in source code.
              WHY? Type safety is mandatory for architectural integrity and preventing runtime failures.
              FIX: Correct the underlying type mismatch instead of ignoring the static analysis warning.

      - name: "Mandatory Class Definition"
        mandatory_class_definition: true
        exceptions: ["__init__.py"]
        mandatory_class_definition_violation_message: >
          AES009 MANDATORY_CLASS_DEFINITION: File is missing a class definition.
          WHY? Encapsulation in classes is required for proper dependency injection and contract adherence.
          FIX: Move standalone functions into a class that implements its corresponding domain contract.

      - name: "Line Count Limits"
        min_lines: 10
        exceptions: ["__init__.py"]
        min_lines_violation_message: >
          AES005 FILE_TOO_SHORT: File contains fewer than 10 lines of code.
          WHY? Excessively small files clutter the project structure; logic should be merged into a parent module.
          FIX: Merge this logic into a related module or expand the component with more functionality.
        max_lines: 500
        exceptions: ["__init__.py", "rust_taxonomy_bridge.py", "js_taxonomy_bridge.py"]
        max_lines_violation_message: >
          AES004 FILE_TOO_LARGE: File exceeds the 500-line limit.
          WHY? Large files violate the Single Responsibility Principle and are difficult to maintain or test.
          FIX: Split the module into smaller, more focused files according to their specific roles.

      - name: "Dead Inheritance Bypass"
        dead_inheritance_bypass: true
        exceptions: ["__init__.py"]
        dead_inheritance_bypass_violation_message: >
          AES016 DEAD_INHERITANCE_BYPASS: Hollow class inheritance detected.
          WHY? Empty classes inheriting from contracts are used to bypass architectural enforcement logic.
          FIX: Implement the required contract methods or remove the class if it is unnecessary.
        dead_inheritance_bypass_custom_messages:
          - pattern: "*Contract*"
            message: >
              AES016 CONTRACT_BYPASS: Class '{name}' uses 'Contract' in its name but is empty.
              WHY? Hollow inheritance is a compliance fraud to trick the linter into seeing contract adherence.
              FIX: Implement the contract methods or remove the class if it is not a real contract.
          - pattern: "*Marker*"
            message: >
              AES016 MARKER_BYPASS: Class '{name}' is a marker interface with no content.
              WHY? Marker classes must have at least one attribute or method to be meaningful in this architecture.
              FIX: Add semantic attributes to the marker or delete it if it serves no purpose.
          - pattern: "*Stub*"
            message: >
              AES016 STUB_BYPASS: Stub class '{name}' inheriting from {bases} provides no implementation.
              WHY? Stubs are temporary placeholders that bypass architectural enforcement logic.
              FIX: Replace the stub with real implementation code or remove it.
          - pattern: "*Dummy*"
            message: >
              AES016 DUMMY_BYPASS: Class '{name}' is a dummy placeholder with no logic.
              WHY? Dummy classes sabotage the contract system and provide zero architectural value.
              FIX: Replace with a real implementation that fulfills the domain contract.
          - pattern: "*Fake*"
            message: >
              AES016 FAKE_BYPASS: Class '{name}' is a fake implementation.
              WHY? Inheriting from a contract without providing methods is a fraudulent bypass.
              FIX: Implement the contract methods or remove the fake implementation.

      - name: "Unused Mandatory Imports"
        check_unused_mandatory_imports: true
        check_unused_mandatory_imports_violation_message: >
          AES015 UNUSED_MANDATORY_IMPORT: Mandatory symbols are imported but never used.
          WHY? Importing required layers without usage is an architectural bypass attempt to satisfy rules.
          FIX: Utilize the imported symbols in your logic or remove the mandatory dependency if unnecessary.

    internal:
      - name: "Root_Standards"
        scope: "root"
        mandatory_class_definition: false
        barrel_completeness: false
        suffix_violation_message: >
          AES011 SUFFIX_MISMATCH: Root file is missing the required '_entry' suffix.
          WHY? The root layer is reserved for system entry points (e.g., cli_main_entry.py).
          FIX: Rename the file to include the '_entry' suffix.

      - name: "Taxonomy_Standards"
        scope: "taxonomy"
        barrel_completeness: true
        forbid_internal_all: true
        barrel_completeness_violation_message: >
          AES012 BARREL_COMPLETENESS: __init__.py is missing the __all__ export list.
          WHY? Taxonomy is the system foundation; all public symbols must be explicitly exported via barrels.
          FIX: Add a proper __all__ = [...] list to the __init__.py file for all public classes.
        forbid_internal_all_violation_message: >
          AES013 INTERNAL_ALL_FORBIDDEN: __all__ export list detected in a non-barrel file.
          WHY? Only the __init__.py barrel file should define the layer's public API surface.
          FIX: Remove __all__ from this file and centralize exports in the layer's __init__.py.
        suffix_violation_message: >
          AES011 SUFFIX_MISMATCH: Taxonomy file is missing a required role-based suffix.
          WHY? Suffixes (_vo, _entity, _error, _event) are required to communicate component roles.
          FIX: Rename the file to include the correct suffix matching its architectural purpose.

      - name: "Contract_Standards"
        scope: "contract"
        no_primitives: true
        barrel_completeness: true
        forbid_internal_all: true
        barrel_completeness_violation_message: >
          AES012 BARREL_COMPLETENESS: __init__.py is missing the __all__ export list.
          WHY? Contracts define the public boundary and must explicitly export their interfaces.
          FIX: Add a proper __all__ = [...] list to the __init__.py exposing only public interfaces.
        forbid_internal_all_violation_message: >
          AES013 INTERNAL_ALL_FORBIDDEN: __all__ export list detected in a non-barrel file.
          WHY? Contract consumption must be centralized through the layer's barrel file.
          FIX: Remove __all__ from this file and export the interfaces via the layer's __init__.py.
        suffix_violation_message: >
          AES008 CONTRACT_SUFFIX_MISMATCH: File is missing a required contract-specific suffix.
          WHY? Every contract must communicate its role as a port, protocol, or IO boundary via suffixes.
          FIX: Rename the file with the correct _port, _protocol, or _io suffix.
        no_primitives_violation_message: >
          AES006 PRIMITIVE_USAGE: Raw primitive types detected in a contract interface.
          WHY? Contracts must use domain types to prevent technical implementation details from leaking.
          FIX: Use taxonomy types (_vo or _entity) instead of raw primitives in interface definitions.

      - name: "Capabilities_Standards"
        scope: "capabilities"
        no_primitives: false
        barrel_completeness: true
        forbid_internal_all: true
        barrel_completeness_violation_message: >
          AES012 BARREL_COMPLETENESS: __init__.py is missing the __all__ export list.
          WHY? Explicit exports ensure a clean and predictable API surface for the agent layer.
          FIX: Add a proper __all__ = [...] list to the __init__.py listing all public capabilities.
        forbid_internal_all_violation_message: >
          AES013 INTERNAL_ALL_FORBIDDEN: __all__ export list detected in a non-barrel file.
          WHY? Use case implementations must be exported exclusively through the layer's barrel file.
          FIX: Remove __all__ from this file and centralize the export in the __init__.py.
        suffix_violation_message: >
          AES011 SUFFIX_MISMATCH: Capability file uses an incorrect or forbidden suffix.
          WHY? Capability suffixes must reflect specific use-case roles like _analyzer or _executor.
          FIX: Rename the file to use an allowed capability suffix defined in the architecture.

      - name: "Infrastructure_Standards"
        scope: "infrastructure"
        no_primitives: false
        barrel_completeness: true
        forbid_internal_all: true
        barrel_completeness_violation_message: >
          AES012 BARREL_COMPLETENESS: __init__.py is missing the __all__ export list.
          WHY? Centralized exports in __init__.py allow the agent to discover adapters consistently.
          FIX: Add a proper __all__ = [...] list to the __init__.py listing all public adapters.
        forbid_internal_all_violation_message: >
          AES013 INTERNAL_ALL_FORBIDDEN: __all__ export list detected in a non-barrel file.
          WHY? Technical implementation details should not define their own public API surface.
          FIX: Remove __all__ from this file and centralize exports in the infrastructure barrel.
        suffix_violation_message: >
          AES011 SUFFIX_MISMATCH: Infrastructure file uses a reserved domain or contract suffix.
          WHY? Infrastructure is for technical implementations; domain concepts are strictly forbidden here.
          FIX: Use technical suffixes such as _adapter, _client, or _scanner for these files.

      - name: "Surfaces_Standards"
        scope: "surfaces"
        no_primitives: false
        barrel_completeness: true
        forbid_internal_all: true
        exceptions: ["__init__.py"]
        barrel_completeness_violation_message: >
          AES012 BARREL_COMPLETENESS: __init__.py is missing the __all__ export list.
          WHY? Surfaces are entry points; all public commands and presenters must be explicitly exported.
          FIX: Add a proper __all__ = [...] list to the __init__.py for all public surface classes.
        forbid_internal_all_violation_message: >
          AES013 INTERNAL_ALL_FORBIDDEN: __all__ export list detected in a non-barrel file.
          WHY? Surface submodules must export via the barrel to maintain a clean boundary.
          FIX: Remove __all__ from this file and centralize exports in the surface __init__.py.
        suffix_violation_message: >
          AES011 SUFFIX_MISMATCH: Surface file uses a forbidden suffix or domain type.
          WHY? Surfaces must not define domain concepts; those belong exclusively in the taxonomy layer.
          FIX: Rename to use surface-specific suffixes like _commands, _page, or _handler.

      - name: "Agent_Standards"
        scope: "agent"
        no_primitives: false
        barrel_completeness: true
        forbid_internal_all: true
        exceptions: ["__init__.py"]
        barrel_completeness_violation_message: >
          AES012 BARREL_COMPLETENESS: __init__.py is missing the __all__ export list.
          WHY? The agent is the system's brain; its public API must be explicitly exported to surfaces.
          FIX: Add a proper __all__ = [...] list to the __init__.py for orchestrators and containers.
        forbid_internal_all_violation_message: >
          AES013 INTERNAL_ALL_FORBIDDEN: __all__ export list detected in a non-barrel file.
          WHY? Agent internal submodules must centralize their exports in the layer's barrel file.
          FIX: Remove __all__ from this file and handle the export via the agent __init__.py.
        suffix_violation_message: >
          AES011 SUFFIX_MISMATCH: Agent file uses a forbidden or non-specialized role suffix.
          WHY? We enforce strict role specialization (container, manager, orchestrator, registry, coordinator).
          FIX: Rename the file to use one of the five allowed agent-specific suffixes.

    external:
      - name: "Taxonomy_Relations"
        scope: "taxonomy"
        check_orphan: true
        orphan_violation_message: >
          AES017 ORPHAN_CODE_DETECTION: Taxonomy component is unreachable and unused.
          WHY? Taxonomy must be consumed by Contract, Infra, Capability, or Surface to be alive.
          FIX: Register in __init__.py and import it in at least one consumer layer.

      - name: "Taxonomy_Vo_Relations"
        scope: "taxonomy(vo)"
        allowed_import: ["taxonomy"]
        mandatory_import: null
        exceptions: ["__init__.py"]
        forbidden_import:
          ["taxonomy(entity,error,event)", "agent", "infrastructure", "surfaces", "contract", "capabilities", "root"]
        forbidden_import_violation_message: >
          AES001 IMPORT_LAYER_VIOLATION: Taxonomy VO imported from an outer layer.
          WHY? Taxonomy is the system's foundation and must have zero dependencies on outer layers.
          FIX: Remove all imports to agent, infrastructure, surfaces, contract, or capabilities layers.

      - name: "Contract_Relations"
        scope: "contract"
        check_orphan: true
        orphan_violation_message: >
          AES017 ORPHAN_CODE_DETECTION: Contract has no heirs.
          WHY? A contract without an implementation in Infrastructure, Capabilities, or Agent is dead code.
          FIX: Implement this contract in a corresponding file or remove it.
        exceptions: ["__init__.py"]

      - name: "Capabilities_Relations"
        scope: "capabilities"
        check_orphan: false
        allowed_import: ["taxonomy", "contract"]
        mandatory_import: ["taxonomy", "contract(protocol)"]
        exceptions: ["__init__.py", "dispatch_parser_types.py", "mcp_tool_schema_checker.py", "mandatory_inheritance_checker.py"]
        forbidden_import: ["infrastructure", "surfaces", "agent", "capabilities", "root"]
        mandatory_import_violation_message: >
          AES002 MANDATORY_IMPORT_MISSING: Capability missing taxonomy or contract protocol.
          WHY? Capabilities implement use cases and require domain types and interface protocols.
          FIX: Import the correct _protocol file and inherit from it immediately.

      - name: "Infrastructure_Relations"
        scope: "infrastructure"
        check_orphan: false
        mandatory_import: ["taxonomy", "contract(port)"]
        exceptions: ["__init__.py", "mcp_server_constants.py", "mcp_server_schemas.py"]
        forbidden_import: ["surfaces", "capabilities", "agent", "infrastructure", "root"]
        allowed_import: ["taxonomy", "contract"]
        mandatory_import_violation_message: >
          AES002 MANDATORY_IMPORT_MISSING: Infrastructure missing taxonomy or port import.
          WHY? Infrastructure provides technical services and must implement a port from the contract layer.
          FIX: Import the correct _port contract and inherit from it.
"#;

pub const JS_CONFIG_TEMPLATE: &str = r#"
thresholds:
  score: 100.0
  complexity: 15

output_dir: "output"

adapters:
  - name: "eslint"
    weight: 1.0
  - name: "prettier"
    weight: 1.0
  - name: "tsc"
    weight: 1.0
  - name: "architecture"
    weight: 3.0

ignored_rules: []

ignored_paths:
  - "./output"
  - "/node_modules"
  - "/dist"
  - "/build"
  - "/.next"
  - "/.git"
  - "/.github"
  - "/coverage"
  - "/scratch"

architecture:
  enabled: true
  layers:
    root: 
      path: "src-javascript/"
      recursive: false
      suffix:
        - strict: ["entry"]
    taxonomy:
      path: "src-javascript/taxonomy"
      recursive: true
      suffix:
        - strict: ["vo", "entity", "error", "event", "bridge"]
    contract:
      recursive: true
      path: "src-javascript/contract"
      suffix:
        - strict: ["port", "protocol", "aggregate"]
    capabilities:
      recursive: true
      path: "src-javascript/capabilities"
      suffix:
        - flexible:
            [
              "analyzer", "actions", "formatters", "generator", "processor",
              "evaluator", "checker", "validator", "transformer", "calculator",
              "builder", "handler", "executor", "resolver", "compiler",
              "aggregator", "classifier", "extractor", "reporter", "mapper",
              "filter", "collector", "comparator", "scorer", "inspector",
              "reviewer", "assessor",
            ]
        - forbidden: ["vo", "entity", "error", "event", "port", "protocol", "io"]
    infrastructure:
      recursive: true
      path: "src-javascript/infrastructure"
      suffix:
        - flexible:
            [
              "adapter", "provider", "scanner", "client", "tracer", "tracker",
              "variants", "detector", "patterns", "util", "system", "repository",
              "cache", "store", "loader", "writer", "reader", "driver",
              "connector", "gateway", "serializer", "encoder", "decoder",
              "fetcher", "watcher", "indexer", "dispatcher", "recorder",
              "proxy", "publisher", "subscriber", "listener", "poller", "streamer",
            ]
        - forbidden: ["vo", "entity", "error", "event", "port", "protocol", "io"]
    surfaces:
      recursive: true
      path: "src-javascript/surfaces"
      suffix:
        - strict: [
            "page", "handler", "controller", "router",
            "component", "layout", "view", "entry",
            "hook", "store", "provider"
          ]
       
    agent:
      path: "src-javascript/agent"
      recursive: true
      suffix:
        - strict:
            [
              "container", "manager", "orchestrator", "registry", "coordinator"
            ]
"#;

pub const RUST_CONFIG_TEMPLATE: &str = r#"
thresholds:
  score: 100.0
  complexity: 15

output_dir: "output"

adapters:
  - name: "clippy"
    weight: 1.0
  - name: "rustfmt"
    weight: 1.0
  - name: "cargo-audit"
    weight: 1.0
  - name: "architecture"
    weight: 3.0

ignored_rules: []

ignored_paths:
  - "./output"
  - "/target"
  - "/.git"
  - "/.github"
  - "/scratch"
"#;
