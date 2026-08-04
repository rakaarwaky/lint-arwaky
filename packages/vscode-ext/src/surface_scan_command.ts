/** Surface: Smart command — VS Code scan entry points.

Imports: taxonomy VOs + contract_scanner_aggregate only.
Delegates all work to IScannerAggregate. Zero business logic.
*/

import * as vscode from "vscode";
import { ScanRequestVO, createScanRequest } from "@lint-arwaky/shared/src/taxonomy_scan_request_vo";
import { ScanResultVO } from "@lint-arwaky/shared/src/taxonomy_scan_result_vo";
import { ViolationVO } from "@lint-arwaky/shared/src/taxonomy_violation_vo";
import { SeverityVO } from "@lint-arwaky/shared/src/taxonomy_severity_vo";
import { IScannerAggregate } from "@lint-arwaky/shared/src/contract_scanner_aggregate";

const SEVERITY_MAP: Record<string, vscode.DiagnosticSeverity> = {
  [SeverityVO.Critical]: vscode.DiagnosticSeverity.Error,
  [SeverityVO.High]: vscode.DiagnosticSeverity.Error,
  [SeverityVO.Medium]: vscode.DiagnosticSeverity.Warning,
  [SeverityVO.Low]: vscode.DiagnosticSeverity.Information,
  [SeverityVO.Info]: vscode.DiagnosticSeverity.Hint,
};

// ─── Block 1: Class Definition & Constructor ──────────────
export class SurfaceScanCommand {
  constructor(
    private readonly aggregate: IScannerAggregate,
    private readonly diagnostics: vscode.DiagnosticCollection,
    private readonly outputChannel: vscode.OutputChannel,
  ) {}

  // ─── Block 2: Public Contract ────────────────────────────
  async handleScan(): Promise<void> {
    const folder = vscode.workspace.workspaceFolders?.[0];
    if (!folder) {
      vscode.window.showErrorMessage("No workspace open.");
      return;
    }
    await this.runScan(folder.uri.fsPath);
  }

  async handleScanFile(): Promise<void> {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
      vscode.window.showErrorMessage("No active file.");
      return;
    }
    await this.runScan(editor.document.uri.fsPath);
  }

  clear(): void {
    this.diagnostics.clear();
  }

  // ─── Block 3: Utility Methods ────────────────────────────
  private async runScan(targetPath: string): Promise<void> {
    try {
      const request: ScanRequestVO = createScanRequest(targetPath);
      const result = await this.aggregate.execute(request);

      this.publishDiagnostics(result);

      const total = result.violations.length;
      if (total === 0) {
        vscode.window.showInformationMessage("Lint Arwaky: No violations found.");
      } else {
        vscode.window.showWarningMessage(`Lint Arwaky: ${total} violation(s).`);
      }
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      this.outputChannel.appendLine(`ERROR: ${msg}`);
      this.outputChannel.show(true);
      vscode.window.showErrorMessage(msg, "OK");
    }
  }

  private publishDiagnostics(result: ScanResultVO): void {
    const grouped = new Map<string, vscode.Diagnostic[]>();

    for (const v of result.violations) {
      const diag = this.violationToDiagnostic(v);
      const existing = grouped.get(v.file) ?? [];
      existing.push(diag);
      grouped.set(v.file, existing);
    }

    this.diagnostics.clear();

    for (const [file, items] of grouped) {
      this.diagnostics.set(vscode.Uri.file(file), items);
    }
  }

  private violationToDiagnostic(v: ViolationVO): vscode.Diagnostic {
    const range = new vscode.Range(
      new vscode.Position(Math.max(0, v.line - 1), Math.max(0, v.column)),
      new vscode.Position(Math.max(0, v.line - 1), 9999),
    );

    const diag = new vscode.Diagnostic(
      range,
      `[${v.code}] ${v.message.split("\n")[0]}`,
      (SEVERITY_MAP[v.severity] ?? vscode.DiagnosticSeverity.Warning) as vscode.DiagnosticSeverity,
    );
    diag.source = "lint-arwaky";
    diag.code = v.code;
    return diag;
  }
}
