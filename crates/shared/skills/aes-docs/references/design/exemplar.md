# DESIGN Exemplar

A filled DESIGN against the Ecommerce Web surface (PR-ECOMMERCE-004), matching the Template shape exactly.

```markdown
# DESIGN — Ecommerce Web

## Design Overview

The Ecommerce Web surface is a responsive browser storefront plus a seller dashboard, built on a single design-token system shared across shopper and merchant pages. It presents the product catalog, cart, and checkout to shoppers and a product-listing and order-management dashboard to merchants. Every color, spacing, typography, and component choice resolves to a named token, so the surface stays consistent as pages are added and can be re-themed without touching layout code.

## Design Requirements

### DS-WEB-001: Core Color Tokens

| Variant | Use case | Token | Value |
|---------|----------|-------|-------|
| Primary | Brand actions, links, and the "Place Order" CTA | `color.primary` | `#2563EB` |
| On-primary | Text on primary surfaces | `color.on-primary` | `#FFFFFF` |
| Surface | Page and card backgrounds | `color.surface` | `#FFFFFF` |
| On-surface | Body text on surfaces | `color.on-surface` | `#111827` |
| Muted | Secondary text and helper copy | `color.muted` | `#6B7280` |
| Error | Validation and stock-out messages | `color.error` | `#DC2626` |

### DS-WEB-002: Spacing and Layout Grid

| Variant | Use case | Token | Value |
|---------|----------|-------|-------|
| Base unit | All spacing scales from this step | `space.1` | `4px` |
| Card padding | Interior padding of product and order cards | `space.card` | `16px` |
| Page gutter | Left/right page margin on desktop | `space.gutter` | `32px` |
| Content max-width | Centered content column for reader pages | `layout.max-width` | `1200px` |
| Card grid | Product catalog column layout | `layout.grid` | `minmax(240px, 1fr)` |

### DS-WEB-003: Typography Scale

| Variant | Use case | Token | Value |
|---------|----------|-------|-------|
| Display | Product detail page title | `type.display` | `32px / 1.1` |
| Heading | Page and section headings | `type.h1` | `24px / 1.2` |
| Body | Default reading text | `type.body` | `16px / 1.5` |
| Caption | Prices, stock labels, and helper text | `type.caption` | `13px / 1.4` |
| Family | All surface text | `type.family` | `Inter, system-ui, sans-serif` |

### DS-WEB-004: Primary Button

| Variant | Use case | Token | Value |
|---------|----------|-------|-------|
| Default | "Add to Cart" and "Place Order" | `button.primary` | `color.primary` fill, `on-primary` label |
| Disabled | Checkout with empty cart | `button.disabled` | `muted` fill, reduced opacity 0.5 |
| Focus | Keyboard focus ring | `button.focus-ring` | `2px` outline in `color.primary` |
| Min height | Touch target on mobile | `button.min-height` | `44px` |

### DS-WEB-005: Card and List Surfaces

| Variant | Use case | Token | Value |
|---------|----------|-------|-------|
| Product card | Catalog tile with image, price, and stock | `card.product` | `surface` background, `8px` radius |
| Order row | Seller dashboard order list entry | `list.order` | `surface` row, `1px` divider in `color.muted` |
| Empty state | Cart or dashboard with no items | `state.empty` | Centered `muted` caption, `48px` min height |
| Hover | Interactive rows and tiles | `state.hover` | `surface` lightened by `4%` |

### DS-WEB-006: Responsive Breakpoints

| Variant | Use case | Token | Value |
|---------|----------|-------|-------|
| Mobile | Single-column storefront | `bp.mobile` | `< 640px` |
| Tablet | Two-column catalog | `bp.tablet` | `640px–1023px` |
| Desktop | Full catalog grid and dashboard | `bp.desktop` | `≥ 1024px` |
| Safe area | Content insets on narrow screens | `bp.safe` | `16px` side padding below mobile |

## Reference

| Artifact     | Purpose                                         | Link |
|--------------|-------------------------------------------------|------|
| PRD          | Business goals, user stories, and scope         | [PRD.md](PRD.md) |
| Backlog      | Task tracking, epics, and implementation status | [BACKLOG.md](BACKLOG.md) |
| Roadmap      | Timeline, phasing, and dependencies             | [ROADMAP.md](ROADMAP.md) |
| Architecture | System boundaries, tech stack, and infra        | [ARCHITECTURE.md](ARCHITECTURE.md) |
```
