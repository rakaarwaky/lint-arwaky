# HOW TO MAKE FRD.md

**Purpose:** Tell an engineer exactly what to build and tell QA exactly what to test — without a single follow-up question to the author.
**Audience:** Engineers, QA, Tech Lead.
**Scope:** One FRD per feature folder.
**Location:** Inside the feature's directory.
**Length:** 50–1000 lines.

---

## Rules

General rules for writing an FRD. These apply across all sections.

1. **Not a feature → no FRD.** A folder is only a valid feature folder when it contains an `agent_*_orchestrator` file. Folders without an orchestrator are not features and must not carry `FRD.md` or `BACKLOG.md`. Cross-cutting kernel / shared layers (e.g. `modules/shared`) have no orchestrator and therefore have **no `FRD.md` and no `BACKLOG.md`**. Pairing (`spec-without-backlog` / `backlog-without-spec`) applies only to feature folders.
2. **Requirement IDs are the contract.** `FR-<FEATURE-NAME-ID>-NNN`, unique within the feature and stable forever.
3. **A requirement is testable, or it is a wish.** State input, output, business rules, edge cases, error handling.
4. **Nothing about the current build.** Shipped/backlog status is a BACKLOG.md claim with evidence, never an FRD sentence.
5. **One observable behaviour per entry.** A reviewer can check it without asking; its `Test` subsection carries observable behaviour + expected result (never "tests pass"), its `Metric` subsection a falsifiable target value.
6. **No acceptance criteria outside the entry tables.** Each FR entry's `Technical Specification`, `Test`, and `Metric` tables — every filled row — IS the acceptance specification; don't duplicate acceptance prose elsewhere.
7. **No source-file extensions in a spec.** Never name `.py`, `.rs`, `.ts`, `.tsx`.
8. **Spec purity — the FRD is a promise.** The FRD states what the feature must do, never how the current build does it. Implementation identifiers (capability names, protocol trait / interface names, capability method names) appear only under the `Capability`, `Protocol`, and `Method` aspect subsections and in `## Public Interface`; every other subsection states behaviour in caller-observable terms.
9. **Caller-observable identifiers may stay.** Error codes, event names, output destination paths, and similar caller-visible vocabulary belong in the `Input`, `Output`, `Error`, and `Event` subsections; they are part of the promise, not implementation.
10. **Purity test.** If a sentence in any subsection names an implementation identifier that is not one of the `Capability` / `Protocol` / `Method` / Public-Interface identifiers above, or leaks build status, move the identifier or cut the sentence.

A `DESIGN.md` in the feature folder is a legitimate input to enumerating FR IDs, but it is not one of the four required Reference links.

---

## Template

Copy, fill, delete nothing.

```markdown
# FRD — <feature-name>

## Feature Overview

<One diagram and one paragraph: where this feature sits, what calls it, what it calls.>

## Functional Requirements

### FR-<FEATURE-NAME-ID>-001: <Short imperative name>

**Description:** <what it does — one paragraph>

#### Business Logic
- <validation logic>
- <validation logic>
- <validation logic>

#### Edge Cases
- <boundary condition & handling>
- <boundary condition & handling>
- <boundary condition & handling>

#### Technical Specification
| Aspect | Detail |
|--------|--------|
| **Input** | <shape, source> |
| **Output** | <shape, destination> |
| **Protocol** | <protocol trait / interface> |
| **Method** | <capability method> |
| **Event** | <domain event emitted, if any> |
| **Error** | <what fails, what the caller sees> |

#### Test
| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| <context> | <action> | <observable result> |

#### Metric
| Target Metric | Value |
|---------------|-------|
| <metric, e.g. Latency / Success Rate> | <target value, e.g. 200ms / 99.9%> |

### FR-<FEATURE-NAME-ID>-002: <Short imperative name>
### FR-<FEATURE-NAME-ID>-003: <Short imperative name>
### FR-<FEATURE-NAME-ID>-004: <Short imperative name>
### FR-<FEATURE-NAME-ID>-005: <Short imperative name>

## Public Interface

| Aspect     | Detail            |
|------------|-------------------|
| Agent      | <agent name>      |
| Aggregate  | <aggregate name>  |
| Method     | <execute>         |
| Request    | <request>         |
| Response   | <response>        |

## Reference

| Artifact     | Purpose                                         | Link |
|--------------|-------------------------------------------------|------|
| PRD          | <Business goals, user stories, and scope>       | [PRD.md](PRD.md) |
| Backlog      | <Task tracking, epics, and implementation status>| [BACKLOG.md](BACKLOG.md) |
| Roadmap      | <Timeline, phasing, and dependencies>           | [ROADMAP.md](ROADMAP.md) |
| Architecture | <System boundaries, tech stack, and infra>      | [ARCHITECTURE.md](ARCHITECTURE.md) |
```

**Seed rows.** Every FR entry seeds one `**Description:**` paragraph, one `#### Business Logic` list, one `#### Edge Cases` list, one `#### Technical Specification` table carrying exactly the six fixed rows `Input`, `Output`, `Protocol`, `Method`, `Event`, `Error` (each with real content), one `#### Test` table with a 3-column header `Context | When Action | Then Observable Result`, and one `#### Metric` table with a 2-column header `Target Metric | Value`. Never pad a table with a placeholder cell such as `—`; the linter flags an empty or dash-padded cell as `empty_detail_cell`. The `Technical Specification` table's six rows are the fixed set — a missing row or an undeclared extra row fires `entry_missing_row` / `table_extra_columns`.

---

## Section Contract

Every section is mandatory; each explains what to fill and what must not appear.

### Feature Overview

| Aspect | Detail |
|---|---|
| **What it is** | One diagram or ≤ 3 sentences locating the feature. |
| **Length** | One diagram or ≤ 3 sentences. |
| **Must contain** | Where the feature sits, what calls it, what it calls. |
| **Weak** | "This feature does reporting." |
| **Falsifiable** | "Sits in `features/reporting/`; called by `agent_reporting_orchestrator`; calls `storage::append`." |

### Functional Requirements

Each `FR-<FEATURE-NAME-ID>-NNN` entry is an H3 heading followed by a `**Description:**` paragraph and five H4 subsections in the canonical order: `#### Business Logic`, `#### Edge Cases`, `#### Technical Specification`, `#### Test`, `#### Metric`. `Business Logic` and `Edge Cases` carry bullet lists; `Technical Specification` is a single `| Aspect | Detail |` table fixed to the six rows `Input`, `Output`, `Protocol`, `Method`, `Event`, `Error`; `Test` is a `| Context | When Action | Then Observable Result |` table; `Metric` is a `| Target Metric | Value |` table. Every table seeds at least one real data row; `Technical Specification` seeds exactly its six fixed rows and no extra.

| Aspect | Detail |
|---|---|
| **What it is** | `FR-<FEATURE-NAME-ID>-NNN` items, each an H3 heading, a `**Description:**` paragraph, and five H4 subsections in canonical order. |
| **Naming** | Short imperative name after the ID. |
| **Description** | One behaviour — what it does. A paragraph, open-ended. |
| **Business Logic** | Validation logic, constraints. Bullet list, open-ended. |
| **Edge Cases** | Boundary conditions and their handling. Bullet list, open-ended. |
| **Technical Specification** | One `| Aspect | Detail |` table, fixed to six rows: `Input` (shape, source), `Output` (shape, destination), `Protocol` (protocol trait / interface), `Method` (capability method), `Event` (domain event emitted, if any), `Error` (what fails, what the caller sees). |
| **Test** | `| Context | When Action | Then Observable Result |` table; observable behaviour + expected result, never "tests pass". Open-ended. |
| **Metric** | `| Target Metric | Value |` table; falsifiable target value. Open-ended. |
| **Seed minimums** | `Technical Specification`: exactly its six fixed rows, each with real content. `Test` / `Metric`: at least one data row. No placeholder cells such as `—`. |
| **Must NOT contain** | Source extensions (`.py`, `.rs`, `.ts`, `.tsx`), build status. |

### Public Interface

| Aspect | Detail |
|---|---|
| **What it is** | Agent / Aggregate / Method / Request / Response surface. |
| **Agent** | `<agent name>` |
| **Aggregate** | `<aggregate name>` |
| **Method** | `<execute>` |
| **Request** | `<request>` |
| **Response** | `<response>` |
| **Must NOT contain** | Internals of those calls. |

### Reference

| Aspect | Detail |
|---|---|
| **What it is** | Connects the FRD to PRD, Backlog, Roadmap, Architecture. |
| **Must contain** | Exactly four rows: PRD, Backlog, Roadmap, Architecture. |
| **PRD** | Business goals, user stories, scope. Links to PRD.md. |
| **Backlog** | Task tracking, epics, implementation status. Links to BACKLOG.md. |
| **Roadmap** | Timeline, phasing, dependencies. Links to ROADMAP.md. |
| **Architecture** | System boundaries, tech stack, infra. Links to ARCHITECTURE.md. |
| **Must NOT contain** | Additional links beyond the four artifacts. |

---

## Exemplar

A filled FRD against the Order Checkout feature, matching the Template shape exactly.

```markdown
# FRD — Order Checkout

## Feature Overview

    [Client] --> [API Gateway] --> [Order Service]
    [Order Service] --> [Inventory Service]
    [Order Service] --> [Pricing / Voucher Service]
    [Order Service] --> [Event Bus]
    [Event Bus] --> [Payment Service]
    [Event Bus] --> [Notification Service]

The **Order Checkout** feature sits in the application/service layer and is called by clients through the API Gateway. This feature is responsible for creating an order from the user's cart, validating item availability, applying vouchers or discounts, and producing an order with an initial status of `PENDING_PAYMENT`. After the order is successfully created, the feature interacts with the Inventory Service to reserve stock and publishes the `OrderCreated` domain event to the Event Bus so that payment, notification, and downstream fulfillment processes can continue asynchronously.

## Functional Requirements

### FR-ORDER-CHECKOUT-001: Create Order

**Description:**
This feature creates a new order based on the cart belonging to an authenticated user. The system validates the cart, checks stock availability, calculates the total amount, applies a voucher if provided, and stores the order with an initial status of `PENDING_PAYMENT`. Once the order is successfully created, the system publishes the `OrderCreated` domain event to trigger downstream processes such as payment, notification, and fulfillment.

#### Business Logic
- The user must be authenticated before performing checkout.
- The cart must not be empty and all items must be valid.
- The quantity of each item must be greater than zero and must not exceed available stock.
- The order total is calculated as subtotal + shipping fee - discount.
- The request must be idempotent using an `idempotencyKey`.
- The order is created only if cart, stock, pricing, and voucher validations are successful.

#### Edge Cases
- If stock becomes unavailable during checkout, the system rejects the order and returns an `OUT_OF_STOCK` error.
- If a duplicate request is submitted with the same `idempotencyKey`, the system does not create a new order and returns the existing order instead.
- If a voucher expires during the checkout process, the order is still created without discount and the response includes a warning.
- If the pricing service times out, the system performs a limited retry or returns a `PRICE_CALCULATION_FAILED` error.

#### Technical Specification
| Aspect | Detail |
|--------|--------|
| **Input** | `CreateOrderRequest` containing `userId`, `cartId`, `idempotencyKey`, and optional `voucherCode`. Source: API Gateway. |
| **Output** | `OrderResponse` containing `orderId`, `status`, `totalAmount`, and `createdAt`. Destination: API Gateway. |
| **Protocol** | `IOrderServiceProtocol` |
| **Method** | `createOrder` |
| **Event** | `OrderCreated` |
| **Error** | `INVALID_CART` 422, `OUT_OF_STOCK` 409, `DUPLICATE_REQUEST` 200 with existing order, `PRICE_CALCULATION_FAILED` 502, `INTERNAL_ERROR` 500 |

#### Test
| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| An authenticated user has a valid cart | The user calls the create order endpoint | An order is created with status `PENDING_PAYMENT` and the `OrderCreated` event is published |
| The cart contains an item with insufficient stock | The user performs checkout | The system rejects the order and returns an `OUT_OF_STOCK` error |
| A duplicate request is submitted with the same `idempotencyKey` | The user resubmits the checkout request | The system does not create a new order and returns the existing order |
| A valid voucher is used during checkout | The user applies the voucher to the order | The order total is reduced according to the discount and the order is still created |

#### Metric
| Target Metric | Value |
|---------------|-------|
| p95 Latency | 250ms |
| Success Rate | 99.9% |
| Duplicate Order Rate | 0% |
| Order Creation Error Rate | < 0.1% |

---

### FR-ORDER-CHECKOUT-002: Reserve Stock

**Description:**
This feature performs temporary stock reservation for a newly created order. Reservation is required so that items currently in the payment process are not sold to another user. If reservation is successful, available stock is temporarily reduced and the order can proceed to payment. If reservation fails, the order must be rejected or marked as failed.

#### Business Logic
- Stock reservation is performed only for items included in the order.
- Available stock must be temporarily reduced while the reservation is active.
- The reservation has a validity period of 15 minutes.
- If payment is not completed within the validity period, the reservation must be canceled.
- Stock reservation may only be invoked by the internal Order Service.

#### Edge Cases
- If stock is insufficient when the reservation is processed, the system fails the order and publishes the `OrderRejected` event.
- If two users attempt to purchase the last remaining item at the same time, only one order successfully reserves the stock.
- If the Inventory Service times out, the system retries up to two times with backoff.
- If the reservation expires, stock is returned and the order is marked as `EXPIRED`.

#### Technical Specification
| Aspect | Detail |
|--------|--------|
| **Input** | `ReserveStockCommand` containing `orderId` and the list of items. Source: Order Service. |
| **Output** | `ReserveStockResult` containing `reservationId`, `status`, and `expiredAt`. Destination: Order Service. |
| **Protocol** | `IInventoryProtocol` |
| **Method** | `reserveStock` |
| **Event** | `StockReserved`, `StockReservationFailed` |
| **Error** | `INSUFFICIENT_STOCK` 409, `INVENTORY_TIMEOUT` 504, `INTERNAL_ERROR` 500 |

#### Test
| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| Items are available in sufficient quantity | The system calls reserve stock | Reservation succeeds and available stock is temporarily reduced |
| Items are not available in sufficient quantity | The system calls reserve stock | Reservation fails and the `StockReservationFailed` event is published |
| The reservation exceeds the 15-minute validity period | The user does not complete payment | The reservation is canceled and stock is returned |
| Two orders compete for the last remaining item | Both orders attempt reservation concurrently | Only one order successfully reserves the stock |

#### Metric
| Target Metric | Value |
|---------------|-------|
| p99 Latency | 150ms |
| Reservation Success Rate | 99.95% |
| Timeout Rate | < 0.1% |
| Stock Consistency | 100% |

---

### FR-ORDER-CHECKOUT-003: Apply Voucher

**Description:**
This feature validates the voucher used by the user and applies a discount to the order total. The system checks the voucher's active status, validity period, usage quota, minimum purchase requirement, and user eligibility. If the voucher is valid, the system returns the discount amount that should be applied to the order.

#### Business Logic
- The voucher must be active and must not be expired.
- The voucher can only be used if the minimum purchase requirement is met.
- A single user may use the same voucher only once.
- The discount must not make the order total negative.
- The voucher is only applied to orders that satisfy the applicable category or campaign conditions.

#### Edge Cases
- If the voucher has expired, the system returns a `VOUCHER_EXPIRED` error.
- If the voucher quota is exhausted during checkout, the system returns a `VOUCHER_QUOTA_EXCEEDED` error.
- If the cart total changes and no longer meets the minimum purchase requirement, the voucher is not applied.
- If the voucher is not valid for the current user, the system returns a `VOUCHER_NOT_ELIGIBLE` error.

#### Technical Specification
| Aspect | Detail |
|--------|--------|
| **Input** | `ApplyVoucherRequest` containing `userId`, `cartId`, and `voucherCode`. Source: Checkout API. |
| **Output** | `VoucherResult` containing `voucherId`, `discountAmount`, and `finalAmount`. Destination: Checkout flow. |
| **Protocol** | `IVoucherProtocol` |
| **Method** | `applyVoucher` |
| **Event** | `VoucherApplied` (optional for analytics/audit) |
| **Error** | `VOUCHER_EXPIRED` 422, `VOUCHER_NOT_ELIGIBLE` 422, `VOUCHER_QUOTA_EXCEEDED` 409, `VOUCHER_INVALID` 400 |

#### Test
| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| The user has an active voucher and the cart meets the minimum purchase requirement | The user applies the voucher | The discount is applied and the order total is reduced |
| The voucher has expired | The user applies the voucher | The system rejects the voucher and returns a `VOUCHER_EXPIRED` error |
| The voucher has already been used by the same user | The user attempts to use the same voucher again | The system rejects the voucher and returns a `VOUCHER_NOT_ELIGIBLE` error |
| The voucher quota has been exhausted | The user applies the voucher | The system rejects the voucher and returns a `VOUCHER_QUOTA_EXCEEDED` error |

#### Metric
| Target Metric | Value |
|---------------|-------|
| p95 Latency | 180ms |
| Voucher Validation Success Rate | 99.9% |
| Invalid Voucher Handling Accuracy | 100% |
| Discount Calculation Accuracy | 100% |

---

## Public Interface

| Aspect     | Detail            |
|------------|-------------------|
| Agent      | AgentOrderOrchestrator |
| Aggregate  | `IOrderAgregate` |
| Method     | `createOrder` |
| Request    | `OrderRequest` |
| Response   | `OrderResponse` |

---

## Reference

| Artifact     | Purpose                                         | Link |
|--------------|-------------------------------------------------|------|
| PRD          | Business goals, user stories, and scope         | [PRD.md](PRD.md) |
| Backlog      | Task tracking, epics, and implementation status | [BACKLOG.md](BACKLOG.md) |
| Roadmap      | Timeline, phasing, and dependencies             | [ROADMAP.md](ROADMAP.md) |
| Architecture | System boundaries, tech stack, and infra        | [ARCHITECTURE.md](ARCHITECTURE.md) |
```

---

## Workflow

Follow these steps in order. The output must match the Template exactly.

1. **Declare the FR ID set first** — enumerate which `FR-<FEATURE-NAME-ID>-NNN` entries the feature needs and why, derived from the feature's deliverables and roadmap promises. The count N is decided here, explicitly, not guessed later. State the total N in the workflow.
2. **Fan out one delegate subagent per FR ID** — spawn exactly N subagents, one per FR ID; each delegate researches its own requirement (the feature's docs: DESIGN, BACKLOG, PRD plus architecture context) and writes ONLY its own `### FR-<FEATURE-NAME-ID>-NNN` heading plus its `**Description:**` paragraph and its five H4 subsections — `Business Logic`, `Edge Cases`, `Technical Specification`, `Test`, `Metric` — seeding each per the **Seed rows** contract in the Template section. If the FRD needs 15 FR IDs, spawn 15 subagents — one per ID, never one agent writing many entries.
3. **Integrate** — merge the N drafts into `## Functional Requirements` in ID order; the integrator polishes against the Rules above (testable, ID contract, no source extensions, no status leaks, spec purity) and does not invent entries for IDs that were never declared. Fill Feature Overview, Public Interface and Reference here too.
4. **Verify** — run `lint-arwaky-cli docs .` plus the parity/structure checks listed in the Verify section.

---

## Verify

```bash
lint-arwaky-cli docs .
```
