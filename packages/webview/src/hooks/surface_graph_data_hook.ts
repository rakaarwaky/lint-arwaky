/** Passive Surface: Hook to receive messages from VS Code extension. */

import { useEffect } from 'react';
import { DependencyGraphVO, ExtensionToWebviewMessage } from '../taxonomy_webview_vo';

interface UseGraphDataProps {
  setGraph: (graph: DependencyGraphVO | null) => void;
  setStatus: (status: 'scanning' | 'complete' | 'error') => void;
  setMessage: (message: string) => void;
}

export function useGraphData({ setGraph, setStatus, setMessage }: UseGraphDataProps): void {
  useEffect(() => {
    const handleMessage = (event: MessageEvent) => {
      const message = event.data as ExtensionToWebviewMessage;
      switch (message.command) {
        case 'scanProgress':
          setStatus(message.status);
          if (message.message) setMessage(message.message);
          break;
        case 'showDependencyGraph':
          setGraph(message.graph);
          setStatus('complete');
          break;
        case 'emptyState':
          setGraph(null);
          setStatus('error');
          setMessage(message.message || `Reason: ${message.reason}`);
          break;
      }
    };

    window.addEventListener('message', handleMessage);
    return () => window.removeEventListener('message', handleMessage);
  }, [setGraph, setStatus, setMessage]);
}
