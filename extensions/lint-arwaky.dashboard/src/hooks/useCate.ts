import { useState, useEffect, useCallback } from 'react';
import type { ScanResults } from '../types';

const STORAGE_KEY = 'scan-results';
const THEME_KEY = 'theme';

type Theme = 'dark' | 'light';

export function useCate() {
  const [isReady, setIsReady] = useState(false);
  const [theme, setThemeState] = useState<Theme>(() => {
    return (localStorage.getItem(THEME_KEY) as Theme) || 'dark';
  });
  const [workspace, setWorkspace] = useState<{ branch: string | null } | null>(null);

  const setTheme = useCallback((t: Theme) => {
    setThemeState(t);
    localStorage.setItem(THEME_KEY, t);
    window.cate?.storage.set(THEME_KEY, t).catch(() => {});
  }, []);

  useEffect(() => {
    const cate = window.cate;

    if (cate) {
      // Cate extension mode — read theme and workspace from extension
      Promise.all([
        cate.theme.get().then((t) => setThemeState(t.type)).catch(() => {}),
        cate.workspace.get().then(setWorkspace).catch(() => {}),
      ]).then(() => setIsReady(true));

      const unsubscribe = cate.storage.onChange?.((key) => {
        if (key === 'theme') {
          cate.theme.get().then((t) => setThemeState(t.type)).catch(() => {});
        }
      });

      return () => { unsubscribe?.(); };
    }

    // Standalone mode — fetch workspace from server
    fetch('/api/workspace')
      .then((r) => r.json())
      .then(setWorkspace)
      .catch(() => {})
      .finally(() => setIsReady(true));
  }, []);

  return { isReady, theme, setTheme, workspace };
}

export function useScanResults() {
  const [results, setResults] = useState<ScanResults | null>(null);
  const [isLoading, setIsLoading] = useState(false);

  // Load from storage on mount
  useEffect(() => {
    const loadResults = async () => {
      const cate = window.cate;
      if (cate) {
        try {
          const stored = await cate.storage.get<ScanResults>(STORAGE_KEY);
          if (stored) setResults(stored);
        } catch (e) {
          console.error('Failed to load from storage:', e);
        }
      } else {
        // Fallback to localStorage
        const stored = localStorage.getItem(STORAGE_KEY);
        if (stored) {
          try {
            setResults(JSON.parse(stored));
          } catch (e) {
            console.error('Failed to parse localStorage:', e);
          }
        }
      }
    };

    loadResults();
  }, []);

  // Listen for storage changes
  useEffect(() => {
    const cate = window.cate;
    if (!cate?.storage.onChange) return;

    const unsubscribe = cate.storage.onChange((key) => {
      if (key === STORAGE_KEY) {
        cate.storage.get<ScanResults>(STORAGE_KEY).then(setResults);
      }
    });

    return () => unsubscribe();
  }, []);

  const saveResults = useCallback(async (newResults: ScanResults) => {
    setResults(newResults);
    
    const cate = window.cate;
    if (cate) {
      await cate.storage.set(STORAGE_KEY, newResults);
    } else {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(newResults));
    }
  }, []);

  const clearResults = useCallback(async () => {
    setResults(null);
    
    const cate = window.cate;
    if (cate) {
      await cate.storage.delete(STORAGE_KEY);
    } else {
      localStorage.removeItem(STORAGE_KEY);
    }
  }, []);

  return { results, isLoading, setIsLoading, saveResults, clearResults };
}
