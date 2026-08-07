export interface ScanViolation {
  file: string;
  line: number;
  column: number;
  rule: string;
  severity: 'error' | 'warning' | 'info';
  message: string;
  group?: string;
}

export interface ScanResults {
  timestamp: string;
  summary: {
    total: number;
    errors: number;
    warnings: number;
    info: number;
  };
  violations: ScanViolation[];
}

export interface RuleGroup {
  name: string;
  count: number;
  color: string;
}

export interface FileViolation {
  file: string;
  errors: number;
  warnings: number;
  info: number;
  total: number;
}

declare global {
  interface Window {
    cate?: {
      version(): Promise<number>;
      panel: {
        id: string;
        setTitle(title: string): Promise<void>;
      };
      workspace: {
        get(): Promise<{ rootPath: string; branch: string | null; worktree: string | null }>;
      };
      theme: {
        get(): Promise<{ id: string; type: 'dark' | 'light'; app: Record<string, string>; terminal: Record<string, string> }>;
      };
      storage: {
        get<T = any>(key: string): Promise<T | undefined>;
        set(key: string, value: any): Promise<void>;
        onChange(cb: (key: string) => void): () => void;
      };
      ui: {
        notify(message: string, level?: 'info' | 'warn' | 'error'): Promise<void>;
      };
    };
  }
}
