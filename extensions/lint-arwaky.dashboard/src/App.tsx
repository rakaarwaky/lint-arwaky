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

export default function App() {
  const { theme, workspace } = useCate();
  const { results, saveResults } = useScanResults();
  const [isScanning, setIsScanning] = useState(false);

  const isDark = theme === 'dark';

  const handleScan = useCallback(async (path: string) => {
    setIsScanning(true);
    
    try {
      // Try to run scan via Cate agent
      if (window.cate?.agent) {
        const session = await window.cate.agent.open();
        if (session.sessionId) {
          const result = await window.cate.agent.send(
            session.sessionId,
            `Run lint scan and return JSON: cargo run --bin lint-arwaky-cli -- scan ${path} --json`
          );
          
          if (result.text) {
            // Try to parse JSON from agent response
            const jsonMatch = result.text.match(/\{[\s\S]*"summary"[\s\S]*"violations"[\s\S]*\}/);
            if (jsonMatch) {
              const scanResults: ScanResults = JSON.parse(jsonMatch[0]);
              scanResults.timestamp = new Date().toISOString();
              await saveResults(scanResults);
              await window.cate.ui.notify(`Scan complete: ${scanResults.summary.total} violations`, 'info');
            }
          }
          
          await window.cate.agent.dispose(session.sessionId);
        }
      } else {
        // Fallback: show instructions
        await window.cate?.ui.notify(
          `Run in terminal: cargo run --bin lint-arwaky-cli -- scan ${path} --json`,
          'info'
        );
      }
    } catch (error: any) {
      console.error('Scan failed:', error);
      await window.cate?.ui.notify(`Scan failed: ${error.message}`, 'error');
    } finally {
      setIsScanning(false);
    }
  }, [saveResults]);

  const handleImport = useCallback(async (data: ScanResults) => {
    await saveResults(data);
    await window.cate?.ui.notify('Scan results imported successfully', 'info');
  }, [saveResults]);

  return (
    <div className="app" data-theme={theme}>
      <Header 
        branch={workspace?.branch ?? null}
        lastScan={results ? formatTime(results.timestamp) : null}
      />
      
      <main className="main">
        <ScanControls onScan={handleScan} isScanning={isScanning} />

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
