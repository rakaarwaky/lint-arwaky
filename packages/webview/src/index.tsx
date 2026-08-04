/** Passive Surface: React entry point for lint-arwaky webview. */

import React from 'react';
import { createRoot } from 'react-dom/client';
import App from './surface_webview_entry';

const container = document.getElementById('root');
if (container) {
  const root = createRoot(container);
  root.render(
    <React.StrictMode>
      <App />
    </React.StrictMode>
  );
}
