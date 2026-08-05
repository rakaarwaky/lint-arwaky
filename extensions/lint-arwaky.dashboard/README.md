# Lint Arwaky Dashboard

Real-time visualization panel for lint-arwaky scan results with React UI.

## Features

- **One-Click Scan** — Run lint-arwaky-cli directly from the dashboard
- **Severity Breakdown** — Errors, warnings, and info counts with visual indicators
- **Rule Group Analysis** — Violations grouped by AES rule categories (naming, import, orphan, quality, role)
- **File Overview** — Per-file violation counts with expandable details
- **Theme Support** — Automatic light/dark mode matching Cate's theme
- **Persistent History** — Scan results stored across sessions

## Setup

```bash
# Install dependencies
npm install

# Build for production
npm run build

# Or run dev server
npm run dev
```

## Usage

1. Build the extension: `npm run build`
2. Sideload the folder in Cate: Settings → Extensions → Add local folder
3. Open the Lint Dashboard panel
4. Click **Scan** or import JSON results manually

## Development

```bash
# Start dev server with hot reload
npm run dev

# Type check
npm run typecheck
```

## Tech Stack

- React 18
- TypeScript
- Vite
- CSS Variables (design tokens)
