import React from 'react';

export function EmptyState() {
  return (
    <div className="empty-state">
      <div className="empty-state-icon">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5">
          <path d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
      </div>
      <h2>No scan results</h2>
      <p>Click "Scan" to analyze your codebase for AES violations</p>
    </div>
  );
}
