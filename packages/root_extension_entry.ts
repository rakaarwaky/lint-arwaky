/** Root: Extension entry — bootstraps the application.

Wires Capabilities to Contract interfaces and starts the VS Code extension.
May depend on all layers.
*/

import * as vscode from "vscode";
import { CliScannerAdapter } from "@lint-arwaky/cli-scanner/src/capabilities_cli_scanner";
import { SurfaceScanCommand } from "@lint-arwaky/vscode-ext/src/surface_scan_command";
import { IScannerProtocol } from "@lint-arwaky/shared/src/contract_scanner_protocol";
import { IScannerAggregate } from "@lint-arwaky/shared/src/contract_scanner_aggregate";
import { ScanRequestVO } from "@lint-arwaky/shared/src/taxonomy_scan_request_vo";
import { ScanResultVO } from "@lint-arwaky/shared/src/taxonomy_scan_result_vo";

// ─── Root Wiring: Protocol → Aggregate Adapter ─────────────
class ScannerAggregateAdapter implements IScannerAggregate {
  constructor(private readonly protocol: IScannerProtocol) {}
  async execute(request: ScanRequestVO): Promise<ScanResultVO> {
    return this.protocol.scan(request);
  }
}

// ─── Bootstrap ────────────────────────────────────────────
export function activate(context: vscode.ExtensionContext): void {
  // Wire: Capabilities (Protocol) → Aggregate → Surface
  const protocol = CliScannerAdapter.create();
  const aggregate = new ScannerAggregateAdapter(protocol);
  const diagnostics = vscode.languages.createDiagnosticCollection("lint-arwaky");
  const outputChannel = vscode.window.createOutputChannel("Lint Arwaky");

  const command = new SurfaceScanCommand(aggregate, diagnostics, outputChannel);

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
