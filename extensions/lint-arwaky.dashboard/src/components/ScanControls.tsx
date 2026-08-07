import React, { useState } from 'react';

interface ScanControlsProps {
  onScan: (path: string) => Promise<void>;
  isScanning: boolean;
}

export function ScanControls({ onScan, isScanning }: ScanControlsProps) {
  const [path, setPath] = useState('.');

  const handleScan = () => {
    onScan(path.trim() || '.');
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !isScanning) {
      handleScan();
    }
  };

  return (
    <section className="scan-controls">
      <div className="scan-left">
        <button 
          className={`btn-scan ${isScanning ? 'scanning' : ''}`}
          onClick={handleScan}
          disabled={isScanning}
        >
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
            <path d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
          </svg>
          <span>{isScanning ? 'Scanning...' : 'Scan'}</span>
        </button>
        <div className="scan-path">
          <label htmlFor="path-input">Path:</label>
          <input
            id="path-input"
            type="text"
            value={path}
            onChange={(e) => setPath(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder="e.g., . or src/"
            disabled={isScanning}
          />
        </div>
      </div>
    </section>
  );
}
