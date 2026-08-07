import { useState, useEffect, useCallback } from 'react';
import type { ScanResults } from '../types';

const STORAGE_KEY = 'scan-results';

export function useCate() {
  const [isReady, setIsReady] = useState(false);
  const [theme, setTheme] = useState<'dark' | 'light'>('dark');
  const [workspace, setWorkspace] = useState<{ branch: string | null } | null>(null);

  useEffect(() => {
    const cate = window.cate;
    if (!cate) {
      setIsReady(true);
      return;
    }

    Promise.all([
      cate.theme.get().then(setTheme).catch(() => {}),
      cate.workspace.get().then(setWorkspace).catch(() => {}),
    ]).then(() => setIsReady(true));

    // Listen for theme changes
    const unsubscribe = cate.storage.onChange?.((key) => {
      if (key === 'theme') {
        cate.theme.get().then(setTheme).catch(() => {});
      }
    });

    return () => {
      unsubscribe?.();
    };
  }, []);

  return { isReady, theme, workspace };
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
