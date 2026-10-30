# HOW TO MAKE PRD.md

**Purpose:** Let any stakeholder state the problem and the exclusions in under 30 seconds — without asking an engineer.
**Audience:** Stakeholders, PM, Design, Engineering leads.
**Scope:** Exactly one per project root.
**Location:** Project root.
**Length:** 50–1000 lines.

---
10→
## Rules

General rules for writing a PRD. These apply across all sections.

1. **Not a feature → no PRD.** A PRD describes a project-level promise. If the entry is a single feature, it belongs in FRD, not here.
2. **No implementation detail.** SQL schemas, API signatures, class names, library choices belong in FRD. If a reader needs the stack to understand the problem, they are reading the wrong file.
3. **No ambiguity.** A PRD must be definitive. Do not leave open questions, assumptions, or "TBD" entries. If you cannot decide, do not write the PRD yet.
4. **Every goal is testable or a wish.** "Fast" is a slogan. "p95 render under 2 s on a 4-core CI runner" is a goal, because a single run can prove it wrong.
5. **Nothing about the current build.** Whether something shipped is a backlog claim with evidence, not a PRD sentence.
20→6. **One observable behaviour per requirement.** Phrased so a reviewer can check it without asking.
7. **No acceptance criteria outside tables.** No test steps anywhere outside the PR aspect tables. Functional test steps belong in FRD.
8. **A PRD is a promise, not a build report.** No status, progress, or "done" language; state belongs in ROADMAP and BACKLOG.
9. **No implementation identifiers.** Code, class, or schema names never appear, unless a caller can observe the identifier directly (a CLI flag, an output line, an exit code).
10. **Purity test.** If a row could be edited by an engineer without a stakeholder noticing, it is not a PRD row. Rewrite it as observable behaviour or delete it.

---

## Template

30→Copy, fill, delete nothing. This is the canonical structure.

```markdown
# PRD — <PROJECT-NAME>

## Project Overview

<One diagram and one paragraph: what the project is, who it is for, why it matters, and what happens if it is not solved.>

## Goals

- <Primary business goal>
- <Primary user goal>
- <Strategic objective>

## Non-Goals

- <Explicitly excluded scope for this project>
- <Related problems not being solved in this release>
- <Future considerations not included now>

## Target Users

| Persona | Need | Current Pain | Desired Outcome |
|---------|------|--------------|-----------------|
| <persona> | <need> | <pain> | <outcome> |

## Scope

### In Scope

- <Major capability or module included in this project>
- <User journey included>
- <Integration included>

### Out of Scope

- <Major capability excluded from this project>
- <User journey excluded>
- <Future enhancement>

---

## Project Requirements

### PR-<PROJECT-NAME-ID>-001: <Short imperative name>

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| <role> | <capability> | <benefit> | <P0/P1/P2> |

#### Proposed Solution

<Describe the proposed solution. Focus on what the user experiences and what the system should achieve.>

#### Main User Flow

1. <Step 1>
2. <Step 2>
3. <Step 3>

### PR-<PROJECT-NAME-ID>-002: <Short imperative name>
### PR-<PROJECT-NAME-ID>-003: <Short imperative name>

## Reference

| Artifact     | Purpose                                          | Link |
|--------------|--------------------------------------------------|------|
| PRD          | <Business goals and scope>                       | [PRD.md](PRD.md) |
| Backlog      | <What is true now, with evidence>                | [BACKLOG.md](BACKLOG.md) |
| Roadmap      | <Timeline, phasing, and dependencies>            | [ROADMAP.md](ROADMAP.md) |
| Architecture | <System boundaries, tech stack, and infra>       | [ARCHITECTURE.md](ARCHITECTURE.md) |
```

Each `### PR-<PROJECT-NAME-ID>-NNN:` entry carries three H4 aspect subsections in canonical order: `#### User Stories`, `#### Proposed Solution`, `#### Main User Flow`. Seed rows follow the seed-row contract in the Section Contract below.

---

## Section Contract

90→Every section is mandatory. Each entry below explains what that section is, how to fill it, and what must not appear.

---

### Project Overview

| Aspect | Detail |
|---|---|
| **What it is** | The reason the project exists. |
| **Length** | Up to 3 paragraphs plus one diagram. |
| **Must contain** | The pain, who feels it, what breaks if nothing changes. |
| **Must NOT contain** | Feature lists, solutions, implementation detail, technical jargon. |

### Goals

| Aspect | Detail |
|---|---|
| **What it is** | Measurable outcomes the project is accountable for. |
| **Length** | One bullet per goal; one bullet each for business, user, strategic. |
| **Must contain** | Testable or a wish. "Fast" is a slogan; a quantifiable target is a goal. |
| **Must NOT contain** | Implementation detail or stack choices. |

### Non-Goals

| Aspect | Detail |
|---|---|
| **What it is** | Explicitly excluded scope so the argument ends here. |
| **Length** | One bullet per exclusion. |
| **Must contain** | Named exclusions only. |
| **Must NOT contain** | "Maybe later" hints that reopen scope. |

### Target Users

| Aspect | Detail |
|---|---|
| **What it is** | One row per persona: who, what they need, the pain, the outcome. |
| **Length** | One table row per persona. |
| **Must contain** | Specific personas enough to disagree with. |
| **Must NOT contain** | Generic "users" without differentiation. |

### Scope

| Aspect | Detail |
|---|---|
| **What it is** | The functional boundary: in scope and out of scope. |
| **Length** | Two H3 subsections (`### In Scope`, `### Out of Scope`), each with a bullet list. |
| **Must contain** | Deliverables and excluded journeys named explicitly. |
| **Must NOT contain** | Future enhancement language inside In Scope. |

---

### Project Requirements

Each requirement is one `### PR-<PROJECT-NAME-ID>-NNN:` heading (the
100→requirement ID is the contract; do not renumber or reorder existing IDs)
followed by three H4 aspect subsections in canonical order: `#### User Stories`,
`#### Proposed Solution`, `#### Main User Flow`. Entries 002 and 003 are elided.

| Subsection | Contract |
|---|---|
| **User Stories** | A `| As a | I want | So that | Priority |` table, one row per persona capability, each priority `P0`/`P1`/`P2`. |
| **Proposed Solution** | One observable behaviour per entry — what it does. No implementation detail, SQL, API signatures, or class names. |
| **Main User Flow** | A numbered list of user-facing steps (1., 2., 3.). |
| **Seed rows** | Each H4 aspect carries at least 1 real content row/bullet/step; write additional rows only when a second real fact exists — never a placeholder cell. This is the `PRD_ASPECT_SEED_MIN` contract (open-ended at 1 per aspect). |

---

### Reference

| Aspect | Detail |
110→|---|---|
| **What it is** | Connects the PRD to its plan, evidence, and technical boundaries. |
| **Must contain** | Exactly four rows: PRD, Backlog, Roadmap, Architecture. |
| **PRD** | Business goals and scope. Links to PRD. |
| **Backlog** | What is true now, with evidence. Links to BACKLOG. |
| **Roadmap** | Timeline, phasing, and dependencies. Links to ROADMAP. |
| **Architecture** | System boundaries, tech stack, infra. Links to ARCHITECTURE. |
| **Must NOT contain** | Additional links, external docs, or implementation diagrams. |

---
120→
## Exemplar

| Template Section  | Weak (do not write)             | Falsifiable (write like this)                                                                                                         |
| ----------------- | ------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| Project Overview  | "Documentation is inefficient." | "Six documents describe one feature and disagree about what is finished; telling whether PR-006 shipped requires reading the source." |
| Goals             | "High quality."                 | "Exit 1 on any status line found in an FRD; exit 0 on a clean tree."                                                                  |
| Non-Goals         | (empty)                         | "Rendering, publishing, or translating docs; the checker only reads and reports."                                                     |
| Target Users      | "Users want docs."              | "As an ops engineer at 3 a.m., I want one file that tells me what shipped, so that I don't grep six sources."                         |
| Scope             | "Everything docs-related."      | "Reads markdown files under /docs and reports invariant violations to stdout."                                                        |

## Complete exemplar: Express Checkout

Below is a complete, folder-friendly exemplar (`PR-CHECKOUT-EXPRESS`) adjusted to the project-name conventions.

---

# PRD — Express Checkout

## Project Overview

```text
[Customer App/Web] --> [API Gateway] --> [Checkout Service]
[Checkout Service] --> [Payment Gateway]
[Checkout Service] --> [Inventory Service]
[Checkout Service] --> [Notification Service]
```

**Express Checkout** is a cross-functional project aimed at reducing friction during the purchase process for returning customers. It targets active shoppers who repeatedly buy from the platform and currently abandon carts due to repetitive checkout steps. The project matters because cart abandonment during checkout is currently at 38%, costing the business an estimated $2.4M in lost revenue per quarter. If this project is not delivered, we will continue losing market share to competitors who already offer one-click purchasing, and customer retention among high-frequency buyers will decline.

## Goals

- Reduce checkout abandonment rate from 38% to below 20% within 6 months of launch.
- Decrease average time-to-purchase from 4 minutes to under 60 seconds for returning customers.
- Increase repeat purchase rate among top-tier loyalty members by 15%.

## Non-Goals

- Redesigning the full shopping or product discovery experience.
- Supporting new payment methods beyond credit cards and digital wallets already integrated.
- Building a merchant-side checkout customization tool (deferred to Q2 next year).
- International shipping or multi-currency support (out of scope for this release).

## Target Users

| Persona | Need | Current Pain | Desired Outcome |
|---------|------|--------------|-----------------|
| Returning Shopper | Buy frequently-used items quickly | Re-entering shipping and payment info every time | Complete a purchase in under 60 seconds with one tap |
| Loyalty Member | Get rewarded for fast, repeat purchases | No streamlined path for high-frequency buyers | Seamless checkout with auto-applied loyalty discounts |
| Mobile Shopper | Checkout on the go with minimal typing | Small screens make form-filling painful | Minimal-tap, mobile-first checkout experience |

## Scope

### In Scope

- Streamlined order checkout flow for authenticated users.
- Saved payment methods with tokenized storage.
- One-click purchase capability for eligible items.
- Auto-application of active vouchers and loyalty points.
- Order confirmation and receipt delivery via email and in-app notification.

### Out of Scope

- Guest checkout flow (handled separately in Guest Checkout project).
- Subscription-based recurring purchases.
- B2B bulk ordering workflows.
- In-store pickup or delivery scheduling.

---

## Project Requirements

### PR-CHECKOUT-EXPRESS-001: Express Checkout

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| Returning customer | to see a pre-filled checkout page with my saved address and payment | I can confirm my order without re-entering details | P0 |
| Returning customer | to review my cart and total before confirming | I feel confident about what I am paying | P0 |
| Customer | to apply or remove a voucher during checkout | I can maximize my savings | P1 |

#### Proposed Solution

The system will present a single-page checkout view for authenticated users, pre-populated with their default shipping address and preferred payment method. Users can modify either before confirming. The order is created atomically, with stock reserved immediately and payment authorized in a single flow. If any validation fails (e.g., out of stock), the user is shown a clear, actionable error with a path to resolution.

#### Main User Flow

1. Customer taps "Checkout" from the cart page.
2. System loads saved address and payment method, displays order summary.
3. Customer reviews total (with auto-applied voucher if eligible) and taps "Place Order".
4. System creates order, reserves stock, authorizes payment, and shows confirmation screen.

---

### PR-CHECKOUT-EXPRESS-002: Saved Payments

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| Returning customer | to save my credit card or digital wallet securely | I do not have to re-enter it on every purchase | P0 |
| Customer | to manage (add, remove, set default) my saved payment methods | I have full control over my payment options | P1 |
| Customer | to see only the last 4 digits of my saved card | I can identify my card without exposing sensitive data | P0 |

#### Proposed Solution

Payment details will be tokenized through the existing payment gateway and stored in a dedicated Payment Vault service. The UI will display masked card information and allow users to add, remove, or set a default method. Tokens are never stored in the application database — only references to the gateway's secure vault.

#### Main User Flow

1. Customer navigates to "Payment Methods" in account settings.
2. Customer taps "Add Payment Method" and completes gateway-hosted card entry.
3. System stores token reference and displays masked card in the list.
4. Customer can set a card as default or remove it at any time.

---

### PR-CHECKOUT-EXPRESS-003: One-Click Buy

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| Loyalty member | to buy an item with a single tap from the product page | I can complete repeat purchases instantly | P0 |
| Customer | to confirm one-click purchases with a biometric or PIN prompt | I feel my account is protected from accidental or unauthorized buys | P0 |
| Customer | to disable one-click purchase at any time | I can control this convenience feature | P1 |

#### Proposed Solution

A "Buy Now" button will appear on eligible product pages for authenticated users with a saved payment method and default address. Tapping it triggers a biometric or PIN confirmation, then creates the order using the same streamlined flow as PR-001. The feature can be toggled off in account settings.

#### Main User Flow

1. Customer taps "Buy Now" on a product page.
2. System prompts biometric or PIN confirmation.
3. Upon confirmation, system creates order, reserves stock, and charges default payment method.
4. Customer sees order confirmation with tracking details.

---

## Reference

| Artifact | Purpose | Link |
|----------|---------|------|
| Roadmap | Timeline, phasing, and dependencies | [ROADMAP.md](ROADMAP.md) |
| Architecture | System boundaries, tech stack, and infra | [ARCHITECTURE.md](ARCHITECTURE.md) |

---

## Workflow

130→Follow these steps in order. The output must match the Template exactly.

1. **Declare the PR ID set first** — enumerate which `PR-<PROJECT-NAME-ID>-NNN` entries the project needs and why they must exist, derived from the project's top-level deliverable folders and roadmap promises. The count N is decided here, explicitly, not guessed later. State the total N in the workflow.
2. **Fan out one delegate subagent per PR ID** — spawn exactly N subagents, one per PR ID; each delegate researches its own subject (the folder's FRD, BACKLOG, and ARCHITECTURE docs plus architecture context) and writes ONLY its own `### PR-<PROJECT-NAME-ID>-NNN` heading plus its three H4 aspect subsections (`#### User Stories`, `#### Proposed Solution`, `#### Main User Flow`), seeded per the seed-row contract. If the PRD needs 15 PR IDs, spawn 15 subagents — one per ID, never one agent writing many entries.
3. **Integrate** — merge the N drafts into `## Project Requirements` in ID order; the integrator polishes against the Rules above (no impl detail, testable metrics, explicit exclusions, open-ended seed rows) and does not invent entries for IDs that were never declared.
4. **Verify** — run `lint-arwaky-cli docs .` and the parity/structure checks already listed in the Verify section.

---

## Verify

```bash
lint-arwaky-cli docs .
140→```
