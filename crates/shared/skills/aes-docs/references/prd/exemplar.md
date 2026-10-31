# PRD Exemplar

```markdown

# PRD — Ecommerce Application

## Project Overview

    [Customer] --> [Storefront] --> [API Gateway]
    [API Gateway] --> [Catalog Service]
    [API Gateway] --> [Checkout Service]
    [API Gateway] --> [Account Service]

**Ecommerce Application** is a cross-functional project that lets customers browse products, add items to a cart, and place orders online. It serves shoppers who buy goods on the platform and merchants who list and sell those goods. The project matters because the platform currently loses customers at checkout and cannot track orders end to end; without it, the business cannot grow its online sales or compete with merchants who offer a complete buying experience.

## Goals

- Reduce cart abandonment from checkout to purchase below 25% within 6 months of launch.
- Enable customers to complete a purchase from product discovery to order confirmation in under 3 minutes.
- Grow merchant gross merchandise value by 20% year over year through the online channel.

## Non-Goals

- Building a brick-and-mortar point-of-sale system.
- Supporting international shipping or multi-currency settlement in this release.
- Offering a marketplace for third-party sellers beyond the platform's own merchant catalog.

## Target Users

| Persona | Need | Current Pain | Desired Outcome |
|---------|------|--------------|-----------------|
| Shopper | Browse products and buy them online | No way to check out or track an order end to end | Find, buy, and receive items with order tracking |
| Merchant | List products and receive orders | No online storefront or order intake | Publish products and get paid orders |
| Customer Service Agent | Look up and resolve order issues | No order history or status tool | View orders and issue refunds or changes |

## Scope

### In Scope

- Product catalog browsing with search and filters.
- Shopping cart with add, update, and remove item actions.
- Authenticated checkout flow with saved addresses and payment.
- Order creation, status tracking, and confirmation emails.
- Merchant product listing and order management dashboard.

### Out of Scope

- Subscription boxes or recurring orders.
- In-store pickup or delivery scheduling.
- Multi-currency pricing and international tax calculation.

---

## Project Requirements

### PR-ECOMMERCE-000: Shared Kernel

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| Developer | to find taxonomy, contract, and utility in one shared layer | I can reuse domain definitions across features | P0 |
| Engineer | to import shared contracts and utilities without circular dependencies | I can build features on stable foundations | P0 |

#### Proposed Solution

The shared layer holds taxonomy value objects, contract protocols and aggregates, and stateless utilities. Feature and surface crates import from shared, but shared imports nothing from them.

#### Main User Flow

1. Developer adds a new value object under `crates/shared/<domain>/`.
2. Feature crate imports the taxonomy type and contract protocol.
3. Surface consumes the contract aggregate to trigger feature behavior.

### PR-ECOMMERCE-001: Product Catalog

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| Shopper | to browse products by category and search by name | I can find the item I want to buy | P1 |
| Shopper | to see product price, stock, and images | I can decide whether to buy | P1 |
| Merchant | to list a product with price and stock | I can sell it on the platform | P1 |

#### Proposed Solution

The storefront displays products from the catalog with category filters and a search bar. Each product page shows price, stock availability, and images. Merchants create and edit product listings through a seller dashboard.

#### Main User Flow

1. Shopper opens the storefront and browses categories or searches for a product.
2. Shopper selects a product to view its details, price, and stock.
3. Merchant adds a product through the seller dashboard with name, price, and stock.

### PR-ECOMMERCE-002: Shopping Cart

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| Shopper | to add items to a cart and adjust quantities | I can buy multiple items in one order | P2 |
| Shopper | to review cart total before checkout | I know what I will pay | P2 |

#### Proposed Solution

The cart holds selected items with quantities and computes a running total. Shoppers can change quantities or remove items before checking out.

#### Main User Flow

1. Shopper taps "Add to Cart" on a product page.
2. Shopper opens the cart to review items, quantities, and total.
3. Shopper adjusts quantity or removes items, then proceeds to checkout.

### PR-ECOMMERCE-003: Checkout and Order

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| Shopper | to check out with saved address and payment | I can place an order without re-entering details | P2 |
| Shopper | to receive an order confirmation | I know my order was placed | P2 |
| Merchant | to receive new orders in the seller dashboard | I can fulfill them | P2 |

#### Proposed Solution

Authenticated shoppers check out through a single-page flow pre-filled with saved address and payment. Placing the order creates an order record, reserves stock, and sends a confirmation. Merchants see new orders in their dashboard.

#### Main User Flow

1. Shopper taps "Checkout" from the cart page.
2. System loads saved address and payment, shows order summary.
3. Shopper reviews total and taps "Place Order".
4. System creates the order, reserves stock, and shows confirmation.

### PR-ECOMMERCE-004: Web

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| Shopper | to browse and buy through a web storefront | I can shop from any device without installing an app | P3 |
| Merchant | to manage products and orders through a web UI | I can run my store from a browser | P3 |

#### Proposed Solution

A web surface presents product catalog, cart, and checkout pages to shoppers and a seller dashboard to merchants. Smart surfaces call contract aggregates to trigger feature behavior, while passive surfaces render presentation only.

#### Main User Flow

1. Shopper opens the web storefront in a browser.
2. Shopper browses products, adds items to cart, and checks out.
3. Merchant opens the seller dashboard to list products and view orders.

## Reference

| Artifact     | Purpose                                         | Link |
|--------------|-------------------------------------------------|------|
| PRD          | Business goals, user stories, and scope         | [PRD.md](PRD.md) |
| Backlog      | Task tracking, epics, and implementation status | [BACKLOG.md](BACKLOG.md) |
| Roadmap      | Timeline, phasing, and dependencies             | [ROADMAP.md](ROADMAP.md) |
| Architecture | System boundaries, tech stack, and infra        | [ARCHITECTURE.md](ARCHITECTURE.md) |

```
