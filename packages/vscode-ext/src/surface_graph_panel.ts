/** Surface: Smart webview provider — manages webview panel lifecycle.

Imports: taxonomy messages + contract IGraphAggregate only.
Delegates all work to aggregates. Zero business logic.
*/

import * as vscode from "vscode";
import { ScanRequestVO, createScanRequest } from "@lint-arwaky/shared/src/taxonomy_scan_request_vo";
import { IScannerAggregate } from "@lint-arwaky/shared/src/contract_scanner_aggregate";
import { IGraphAggregate } from "@lint-arwaky/shared/src/contract_graph_aggregate";
import { WebviewToExtensionMessage } from "@lint-arwaky/shared/src/taxonomy_webview_vo";
import { WebviewManager } from "./surface_webview_manager";

// ─── Block 1: Class Definition & Constructor ──────────────
export class SurfaceGraphPanel implements vscode.WebviewViewProvider {
  public static readonly viewType = "lint-arwaky.graphView";
  private _view?: vscode.WebviewView;
  private readonly _webviewManager: WebviewManager;

  constructor(
    private readonly extensionUri: vscode.Uri,
    private readonly scannerAggregate: IScannerAggregate,
    private readonly graphAggregate: IGraphAggregate,
  ) {
    this._webviewManager = new WebviewManager(extensionUri);
  }

  // ─── Block 2: Public Contract ────────────────────────────
  resolveWebviewView(
    webviewView: vscode.WebviewView,
    _context: vscode.WebviewViewResolveContext,
    _token: vscode.CancellationToken,
  ): void {
    this._view = webviewView;
    webviewView.webview.options = this._webviewManager.getWebviewOptions();
    webviewView.webview.html = this._webviewManager.getHtmlForWebview(webviewView.webview);

    webviewView.webview.onDidReceiveMessage(
      (message: WebviewToExtensionMessage) => this._handleMessage(message),
      undefined,
      [],
    );
  }

  async refreshGraph(): Promise<void> {
    if (!this._view) return;
    const folder = vscode.workspace.workspaceFolders?.[0];
    if (!folder) {
      this._view.webview.postMessage({ command: "emptyState", reason: "no-workspace" });
      return;
    }

    this._view.webview.postMessage({ command: "scanProgress", status: "scanning" });

    try {
      const request: ScanRequestVO = createScanRequest(folder.uri.fsPath);
      const graph = await this.graphAggregate.execute(request);
      this._view.webview.postMessage({ command: "showDependencyGraph", graph });
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      this._view.webview.postMessage({ command: "emptyState", reason: "scan-failed", message: msg });
    }
  }

  // ─── Block 3: Utility Methods ────────────────────────────
  private async _handleMessage(message: WebviewToExtensionMessage): Promise<void> {
    switch (message.command) {
      case "refreshScan":
        await this.refreshGraph();
        break;
      case "openViolation":
        await this._openFile(message.file, message.line, message.column);
        break;
      case "filterSeverity":
      case "filterLayer":
        // Filter handled client-side in webview
        break;
    }
  }

  private async _openFile(file: string, line: number, column: number): Promise<void> {
    const uri = vscode.Uri.file(file);
    const doc = await vscode.workspace.openTextDocument(uri);
    const editor = await vscode.window.showTextDocument(doc);
    const position = new vscode.Position(Math.max(0, line - 1), Math.max(0, column));
    editor.selection = new vscode.Selection(position, position);
    editor.revealRange(new vscode.Range(position, position), vscode.TextEditorRevealType.InCenter);
  }
}
