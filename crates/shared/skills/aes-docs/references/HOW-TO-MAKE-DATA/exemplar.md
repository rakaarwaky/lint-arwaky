# DATA Exemplar

A filled DATA against the Ecommerce shared kernel (DATA-ECOM), matching the Template shape exactly.

```markdown
# DATA — Ecommerce Shared Kernel

## Data Overview

The shared kernel owns the stable domain data shapes that every feature and surface reads and writes: products in the catalog, cart items a shopper selects, orders placed at checkout, shipping addresses, and payment methods. These value objects are immutable domain concepts with no behavior, and no feature crate defines its own copy.

## Data Requirements

### DT-SHARED-001: Product value object

| Type | Description |
|------|-------------|
| Product | A sellable item: name, price, stock quantity, category, and image reference. |

### DT-SHARED-002: CartItem value object

| Type | Description |
|------|-------------|
| CartItem | One product in a shopper's cart with a quantity and a line total. |

### DT-SHARED-003: Order value object

| Type | Description |
|------|-------------|
| Order | A placed order: shopper, items, total, status, and confirmation timestamp. |

### DT-SHARED-004: Address value object

| Type | Description |
|------|-------------|
| Address | A saved shipping or billing address: name, street, city, postal code, country. |

### DT-SHARED-005: PaymentMethod value object

| Type | Description |
|------|-------------|
| PaymentMethod | A stored payment instrument: type, last four digits, and expiry. |

## Reference

| Artifact     | Purpose                                         | Link|
|--------------|-------------------------------------------------|------|
| PRD          | Business goals, user stories, and scope         | [PRD.md](PRD.md)|
| Backlog      | Task tracking, epics, and implementation status | [BACKLOG.md](BACKLOG.md)|
| Roadmap      | Timeline, phasing, and dependencies             | [ROADMAP.md](ROADMAP.md)|
| Architecture | System boundaries, tech stack, and infra        | [ARCHITECTURE.md](ARCHITECTURE.md)|

```
