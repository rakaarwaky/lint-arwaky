import React, { useState } from 'react';
import type { ScanResults } from '../types';

interface ImportSectionProps {
  onImport: (results: ScanResults) => void;
}

export function ImportSection({ onImport }: ImportSectionProps) {
  const [text, setText] = useState('');
  const [error, setError] = useState<string | null>(null);

  const handleImport = () => {
    setError(null);
    
    if (!text.trim()) {
      setError('Please paste JSON data');
      return;
    }

    try {
      const data = JSON.parse(text);

      // Validate structure
      if (!data.summary || !Array.isArray(data.violations)) {
        throw new Error('Invalid format: missing summary or violations');
      }

      // Add timestamp if missing
      if (!data.timestamp) {
        data.timestamp = new Date().toISOString();
      }

      onImport(data);
      setText('');
    } catch (e: any) {
      setError(e.message || 'Invalid JSON format');
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && e.metaKey) {
      handleImport();
    }
  };

  return (
    <section className="import-section">
      <h3>Import Scan Results (Manual)</h3>
      <div className="import-box">
        <textarea
          value={text}
          onChange={(e) => setText(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder={`Paste JSON scan results here...\n\nExample:\n{\n  "summary": { "total": 10, "errors": 2, "warnings": 5, "info": 3 },\n  "violations": [...]\n}`}
        />
        <div className="import-actions">
          <button className="btn-primary" onClick={handleImport}>
            Import
          </button>
          {error && <span className="import-error">{error}</span>}
        </div>
      </div>
    </section>
  );
}
