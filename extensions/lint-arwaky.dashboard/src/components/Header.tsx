import React from 'react';

interface HeaderProps {
  branch: string | null;
  lastScan: string | null;
}

export function Header({ branch, lastScan }: HeaderProps) {
  return (
    <header className="header">
      <div className="header-left">
        <svg className="logo" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
          <path d="M12 2L2 7l10 5 10-5-10-5z" />
          <path d="M2 17l10 5 10-5" />
          <path d="M2 12l10 5 10-5" />
        </svg>
        <h1>Lint Arwaky</h1>
      </div>
      <div className="header-right">
        <span className="badge">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
            <path d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
          </svg>
          <span>{branch || 'No workspace'}</span>
        </span>
        <span className="badge">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
            <circle cx="12" cy="12" r="10" />
            <path d="M12 6v6l4 2" />
          </svg>
          <span>{lastScan || 'No scans yet'}</span>
        </span>
      </div>
    </header>
  );
}
