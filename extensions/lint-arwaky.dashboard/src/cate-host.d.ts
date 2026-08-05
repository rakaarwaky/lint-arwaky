/**
 * Type definitions for Cate Host API (window.cate)
 * @see https://github.com/0-AI-UG/cate-extensions
 */

interface CateTheme {
  id: string;
  type: 'dark' | 'light';
  app: Record<string, string>;
  terminal: Record<string, string>;
}

interface CateWorkspace {
  rootPath: string;
  branch: string | null;
  worktree: string | null;
}

interface CateStorage {
  get<T = any>(key: string): Promise<T | undefined>;
  set(key: string, value: any): Promise<void>;
  delete(key: string): Promise<void>;
  keys(): Promise<string[]>;
  panel: {
    get<T = any>(key: string): Promise<T | undefined>;
    set(key: string, value: any): Promise<void>;
  };
  onChange(cb: (key: string) => void): () => void;
}

interface CateUI {
  notify(message: string, level?: 'info' | 'warn' | 'error'): Promise<void>;
}

interface CatePanel {
  id: string;
  setTitle(title: string): Promise<void>;
  list(): Promise<Array<{
    panelId: string;
    type: string;
    title: string;
    focused: boolean;
    filePath?: string;
    url?: string;
  }>>;
  focus(panelId: string): Promise<void>;
  close(panelId: string): Promise<void>;
}

interface CateEditor {
  openFile(path: string, options?: { line?: number; column?: number }): Promise<void>;
}

interface CateCanvas {
  createPanel(type: 'browser' | 'editor' | 'extension', options?: {
    position?: { x: number; y: number };
    url?: string;
    filePath?: string;
    extensionId?: string;
    extensionPanelId?: string;
  }): Promise<{ panelId: string }>;
}

interface CateFiles {
  onDrop(cb: (files: Array<{
    name: string;
    path: string | null;
    text: string;
    size?: number;
    truncated?: boolean;
  }>) => void): () => void;
}

interface CateAgent {
  open(options?: { resume?: boolean }): Promise<{ sessionId: string } | { error: string }>;
  send(sessionId: string, prompt: string): Promise<{ text: string; message: any } | { error: string }>;
  dispose(sessionId: string): Promise<void>;
  cancel(): Promise<void>;
}

interface CateBrowser {
  open(options: { url: string; panelId?: string }): Promise<{ panelId: string; url: string }>;
  reload(options?: { panelId?: string }): Promise<{ ok: true }>;
  screenshot(options?: { panelId?: string }): Promise<{ path: string }>;
  snapshot(options?: { panelId?: string }): Promise<{
    url: string;
    title: string;
    refs: Array<{
      ref: string;
      role: string;
      name: string;
      value?: string;
    }>;
  }>;
  click(options: { ref: string; panelId?: string }): Promise<{ ok: true }>;
  type(options: { ref: string; text: string; panelId?: string }): Promise<{ ok: true }>;
  wait(options?: { panelId?: string; timeoutMs?: number }): Promise<{
    url: string;
    title: string;
    loading: boolean;
  }>;
  press(options: { key: string; ref?: string; panelId?: string }): Promise<{ ok: true }>;
}

interface Cate {
  version(): Promise<number>;
  panel: CatePanel;
  workspace: {
    get(): Promise<CateWorkspace>;
  };
  theme: {
    get(): Promise<CateTheme>;
  };
  storage: CateStorage;
  ui: CateUI;
  editor: CateEditor;
  canvas: CateCanvas;
  files: CateFiles;
  agent: CateAgent;
  browser: CateBrowser;
}

declare global {
  interface Window {
    cate?: Cate;
  }
}

export {};
