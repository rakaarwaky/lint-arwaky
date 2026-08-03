import * as vscode from "vscode";
import { scan, CliError } from "./cli-runner";

let diagnostics: vscode.DiagnosticCollection;

const severityMap: Record<string, vscode.DiagnosticSeverity> = {
  critical: vscode.DiagnosticSeverity.Error,
  high: vscode.DiagnosticSeverity.Error,
  medium: vscode.DiagnosticSeverity.Warning,
  low: vscode.DiagnosticSeverity.Information,
  info: vscode.DiagnosticSeverity.Hint,
};

export function activate(context: vscode.ExtensionContext) {
  diagnostics = vscode.languages.createDiagnosticCollection("lint-arwaky");

  context.subscriptions.push(
    vscode.commands.registerCommand("lint-arwaky.scan", handleScan),
    vscode.commands.registerCommand("lint-arwaky.scanFile", handleScanFile),
    diagnostics,
  );
}

async function handleScan() {
  const folder = vscode.workspace.workspaceFolders?.[0];
  if (!folder) {
    vscode.window.showErrorMessage("No workspace open.");
    return;
  }
  await runScan(folder.uri.fsPath);
}

async function handleScanFile() {
  const editor = vscode.window.activeTextEditor;
  if (!editor) {
    vscode.window.showErrorMessage("No active file.");
    return;
  }
  await runScan(editor.document.uri.fsPath);
}

async function runScan(targetPath: string) {
  try {
    const result = await scan(targetPath);
    const grouped = new Map<string, vscode.Diagnostic[]>();

    for (const v of result.results) {
      const fileUri = vscode.Uri.file(v.file);
      const range = new vscode.Range(
        new vscode.Position(Math.max(0, v.line - 1), Math.max(0, v.column)),
        new vscode.Position(Math.max(0, v.line - 1), 9999),
      );

      const diag = new vscode.Diagnostic(
        range,
        `[${v.code}] ${v.message.split("\n")[0]}`,
        severityMap[v.severity] ?? vscode.DiagnosticSeverity.Warning,
      );
      diag.source = "lint-arwaky";
      diag.code = v.code;

      const existing = grouped.get(v.file) ?? [];
      existing.push(diag);
      grouped.set(v.file, existing);
    }

    diagnostics.clear();

    for (const [file, items] of grouped) {
      diagnostics.set(vscode.Uri.file(file), items);
    }

    const total = result.results.length;
    if (total === 0) {
      vscode.window.showInformationMessage("Lint Arwaky: No violations found.");
    } else {
      vscode.window.showWarningMessage(`Lint Arwaky: ${total} violation(s).`);
    }
  } catch (err) {
    if (err instanceof CliError) {
      vscode.window.showErrorMessage(err.message);
    } else {
      vscode.window.showErrorMessage(
        `Lint Arwaky: ${err instanceof Error ? err.message : String(err)}`,
      );
    }
  }
}

export function deactivate() {
  diagnostics.clear();
}
