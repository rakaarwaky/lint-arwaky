# Examples

## BAD: Capability Without Trait (AES403)

```rust
pub struct <NameComposer>;

impl <NameComposer> {
    pub fn compose_frame(&self) {
        // public behavior without protocol trait
    }
}
```

Fix:

```rust
pub struct <NameComposer>;

impl I<NameComposer>Protocol for <NameComposer> {
    fn compose_frame(&self) {
        // contract implementation
    }
}
```

## BAD: I/O in Capabilities (AES404)

```rust
impl <NameCapability> {
    fn process(&self) {
        let content = std::fs::read_to_string("file.txt"); // FORBIDDEN
    }
}
```

Fix: Move I/O to utility.

## BAD: Data Class Defined in Layer File

```rust
pub struct <NameResult> {
    is_valid: bool,
    reason: String,
}
```

Fix: Move to shared taxonomy, then import.

## BAD: Concrete Service Field

```rust
pub struct Capabilities<NameCapability> {
    collaborator: <NameCollaborator>, // BAD
}
```

Fix:

```rust
pub struct Capabilities<NameCapability> {
    collaborator: Arc<dyn I<NameCollaborator>Protocol>,
}
```

## BAD: Std Trait in Block 2

```rust
pub struct Capabilities<NameCapability>;

impl Default for Capabilities<NameCapability> {
    fn default() -> Self { Self }
}

impl I<NameCapability>Protocol for Capabilities<NameCapability> {
    fn execute(&self, ...) { // ...
    }
}
```

Fix: Move `Default` to Block 3.

## BAD: Orchestration Inside Capability (No Orchestration, §8)

```rust
impl <NamePipeline>Protocol for <NamePipeline> {
    fn run(&self) {
        let a = self.step_a();      // calls another capability's behavior
        if a.is_ok() {
            self.step_b();          // branching between capabilities
        } else {
            self.escalate();        // error-escalation policy
        }
    }
}
```

Fix: remove flow control and cross-capability calls. Let the Agent layer compose the pipeline. The capability executes one responsibility and returns a result.

## BAD: Domain Model Defined in Capability (No Domain Definition, §8)

```rust
pub struct <NameResult> {   // domain model defined here = forbidden
    is_valid: bool,
    reason: String,
}
```

Fix: define `<NameResult>` as a Taxonomy VO; the capability only consumes and produces it.

## GOOD: Capability with DI and Shared VO

```rust
use std::sync::Arc;

use shared::<name-feature>::taxonomy_<name>_vo::<Name>VO;
use shared::<name-feature>::contract_<name>_protocol::I<Name>Protocol;
use shared::<name-feature>::taxonomy_<name-collaborator>_protocol::I<NameCollaborator>Protocol;
use shared::<name-feature>::taxonomy_<name-store>_protocol::I<NameStore>Protocol;

pub struct Capabilities<NameCapability> {
    collaborator: Arc<dyn I<NameCollaborator>Protocol>,
    store: Arc<dyn I<NameStore>Protocol>,
    policy: <NamePolicy>VO,
}

impl I<NameCapability>Protocol for Capabilities<NameCapability> {
    // public contract methods only
}
```

## GOOD: Correct 3-Block Structure

```rust
use std::sync::Arc;

use shared::<name-feature>::taxonomy_<domain>_vo::<DomainVO>;
use shared::<name-feature>::contract_<name>_protocol::I<NameCapability>Protocol;
use shared::<name-feature>::taxonomy_<name-utility>::<name>_utility;
use shared::<name-feature>::taxonomy_<result>_vo::<ResultVO>;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct Capabilities<NameCapability>;

// ─── Block 2: Public Contract (domain protocol ONLY) ──────
impl I<NameCapability>Protocol for Capabilities<NameCapability> {
    fn execute(
        &self,
        input: &<DomainVO>,
        output: &mut Vec<<ResultVO>>,
    ) {
        let key = input.key();
        if <name>_utility(key) {
            return;
        }
        // Remaining domain logic...
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl Default for Capabilities<NameCapability> {
    fn default() -> Self { Self }
}

impl Capabilities<NameCapability> {
    pub fn new() -> Self { Self }

    fn is_relevant(&self, input: &<DomainVO>) -> bool {
        // Private helper specific to this capability.
        true
    }
}
```
