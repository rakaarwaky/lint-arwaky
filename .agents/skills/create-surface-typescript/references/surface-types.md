# Surface Types

## Smart Surface

Smart surfaces are entry points or controllers.

Suffixes: `_command`, `_controller`, `_page`, `_entry`

Smart surfaces:

- receive user intent,
- map events to requests/actions,
- delegate to aggregates,
- update UI/application state from aggregate results.

Smart surfaces MUST NOT:

- import capabilities,
- import infrastructure,
- import concrete agents,
- perform I/O,
- compute domain results,
- validate business rules.

## Utility Surface

Utility surfaces are thin adapters between user interaction and smart surfaces.

Suffixes: `_hook`, `_store`, `_action`, `_screen`

Utility surfaces:

- map low-level events to shared action/event VOs,
- hold lightweight UI state,
- compose passive components,
- emit actions for smart surfaces to handle.

Utility surfaces MUST NOT import concrete smart surfaces.

## Passive Surface

Passive surfaces are pure rendering/display components.

Suffixes: `_component`, `_view`, `_layout`

Passive surfaces:

- render from shared VOs,
- display precomputed state,
- may format presentation output,
- may iterate over precomputed view items,
- MUST NOT contain business logic,
- MUST NOT contain domain computation,
- MUST NOT orchestrate aggregates.
