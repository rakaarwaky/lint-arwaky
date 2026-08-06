import React, { useState, useCallback } from 'react';
import { useCate, useScanResults } from './hooks/useCate';
import { Header } from './components/Header';
import { ScanControls } from './components/ScanControls';
import { SummaryCards } from './components/SummaryCards';
import { SeverityChart } from './components/SeverityChart';
import { RuleGroupsChart } from './components/RuleGroupsChart';
import { FileList } from './components/FileList';
import { ImportSection } from './components/ImportSection';
import { EmptyState } from './components/EmptyState';
import { groupByRule, groupByFile, formatTime } from './utils';
import type { ScanResults } from './types';

async function apiFetch<T>(path: string, init?: RequestInit): Promise<T> {
  const res = await fetch(path, init);
  if (!res.ok) {
    const body = await res.json().catch(() => ({}));
    throw new Error(body.message || `Server error ${res.status}`);
  }
  return res.json();
}

export default function App() {
  const { theme, setTheme, workspace } = useCate();
  const { results, saveResults } = useScanResults();
  const [isScanning, setIsScanning] = useState(false);
  const [scanError, setScanError] = useState<string | null>(null);

  const isDark = theme === 'dark';

  const handleScan = useCallback(async (path: string) => {
    setIsScanning(true);
    setScanError(null);

    try {
      const data = await apiFetch<ScanResults & { error?: string; message?: string; raw?: string; note?: string }>(
        '/api/scan',
        {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ path }),
        }
      );

      if (data.error) {
        setScanError(data.message || data.error);
        return;
      }

      data.timestamp = new Date().toISOString();
      await saveResults(data);
    } catch (error: any) {
      console.error('Scan failed:', error);
      setScanError(error.message);
    } finally {
      setIsScanning(false);
    }
  }, [saveResults]);

  const handleImport = useCallback(async (data: ScanResults) => {
    await saveResults(data);
  }, [saveResults]);

  return (
    <div className="app" data-theme={theme}>
      <Header
        branch={workspace?.branch ?? null}
        lastScan={results ? formatTime(results.timestamp) : null}
        theme={theme}
        onToggleTheme={() => setTheme(isDark ? 'light' : 'dark')}
      />
      
      <main className="main">
        <ScanControls onScan={handleScan} isScanning={isScanning} />

        {scanError && (
          <div className="scan-error-banner">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" width="16" height="16">
              <circle cx="12" cy="12" r="10" /><path d="M12 8v4M12 16h.01" />
            </svg>
            <span>{scanError}</span>
          </div>
        )}

        {!results ? (
          <EmptyState />
        ) : (
          <div className="dashboard">
            <SummaryCards
              total={results.summary.total}
              errors={results.summary.errors}
              warnings={results.summary.warnings}
              info={results.summary.info}
            />

            <section className="charts-row">
              <SeverityChart
                errors={results.summary.errors}
                warnings={results.summary.warnings}
                info={results.summary.info}
                isDark={isDark}
              />
              <RuleGroupsChart groups={groupByRule(results.violations)} />
            </section>

            <FileList files={groupByFile(results.violations)} />

            <ImportSection onImport={handleImport} />
          </div>
        )}
      </main>
    </div>
  );
}
