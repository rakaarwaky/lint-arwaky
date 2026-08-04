/** Root: Extension entry — bootstraps the application.

Wires Capabilities to Contract interfaces and starts the VS Code extension.
May depend on all layers.
*/

import * as vscode from "vscode";
import { CliScannerAdapter } from "@lint-arwaky/cli-scanner/src/capabilities_cli_scanner";
import { SurfaceScanCommand } from "@lint-arwaky/vscode-ext/src/surface_scan_command";

// ─── Bootstrap ────────────────────────────────────────────
export function activate(context: vscode.ExtensionContext): void {
  // Wire: Capabilities → Contract → Surface
  const scanner = CliScannerAdapter.create();
  const diagnostics = vscode.languages.createDiagnosticCollection("lint-arwaky");
  const outputChannel = vscode.window.createOutputChannel("Lint Arwaky");

  const command = new SurfaceScanCommand(scanner, diagnostics, outputChannel);

  context.subscriptions.push(
    vscode.commands.registerCommand("lint-arwaky.scan", () => command.handleScan()),
    vscode.commands.registerCommand("lint-arwaky.scanFile", () => command.handleScanFile()),
    diagnostics,
    outputChannel,
  );
}

export function deactivate(): void {
  // cleanup handled by VS Code via disposables
}
