# PRD Exemplar

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
