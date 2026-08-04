/** Capabilities: CLI scanner adapter — implements IScannerProtocol.

Block 1: Class Definition & Constructor
Block 2: Protocol Method Implementation
Block 3: Utility Methods, Factories & Helpers
*/

import * as vscode from "vscode";
import { execFile } from "node:child_process";
import { promisify } from "node:util";
import { whichSync } from "@lint-arwaky/shared/src/utility_which_resolver";
import { CliError } from "@lint-arwaky/shared/src/taxonomy_cli_error";
import { ScanRequestVO } from "@lint-arwaky/shared/src/taxonomy_scan_request_vo";
import { ScanResultVO } from "@lint-arwaky/shared/src/taxonomy_scan_result_vo";
import { IScannerProtocol } from "@lint-arwaky/shared/src/contract_scanner_protocol";

const execFileAsync = promisify(execFile);

// ─── Block 1: Class Definition & Constructor ──────────────
export class CliScannerAdapter implements IScannerProtocol {
  constructor(
    private readonly cliPath: string = "lint-arwaky-cli",
  ) {}

  // ─── Block 2: Protocol Method Implementation ─────────────
  async scan(request: ScanRequestVO): Promise<ScanResultVO> {
    const resolved = this.resolveCliPath();
    const args = this.buildArgs(request);

    const { stdout, stderr } = await this.executeCli(resolved, args);
    if (stderr) {
      console.warn(`[lint-arwaky] stderr: ${stderr}`);
    }
    return this.parseOutput(stdout, stderr);
  }

  // ─── Block 3: Utility Methods, Factories & Helpers ────────
  private resolveCliPath(): string {
    const config = vscode.workspace.getConfiguration("lint-arwaky");
    const configured = config.get<string>("cliPath", this.cliPath);
    const resolved = whichSync(configured);

    if (!resolved) {
      const installCmd =
        configured === "lint-arwaky-cli"
          ? "cargo install lint-arwaky-cli"
          : `Install ${configured} and add to PATH`;

      throw new CliError(
        `lint-arwaky-cli not found in PATH.\n\nInstall it first:\n  ${installCmd}`,
        null,
        "",
      );
    }

    return resolved;
  }

  private buildArgs(request: ScanRequestVO): string[] {
    const args = ["scan", request.targetPath, "--format", "json"];
    if (request.language) {
      args.push("--language", request.language);
    }
    return args;
  }

  private async executeCli(
    cliPath: string,
    args: string[],
  ): Promise<{ stdout: string; stderr: string }> {
    try {
      const result = await execFileAsync(cliPath, args, {
        maxBuffer: 10 * 1024 * 1024,
        timeout: 120_000,
      });
      return { stdout: result.stdout, stderr: result.stderr };
    } catch (err: unknown) {
      if (err instanceof Error && "stdout" in err) {
        const nodeErr = err as unknown as {
          stdout?: string;
          stderr?: string;
          code: number;
        };
        const stdout = nodeErr.stdout ?? "";
        const stderr = nodeErr.stderr ?? "";

        if (nodeErr.code > 1) {
          throw new CliError(
            `lint-arwaky-cli failed (code ${nodeErr.code}):\n${stderr || "unknown error"}`,
            nodeErr.code,
            stderr,
          );
        }
        return { stdout, stderr };
      }
      throw err;
    }
  }

  private parseOutput(stdout: string, stderr: string): ScanResultVO {
    if (!stdout.trim()) {
      throw new CliError("lint-arwaky-cli returned empty output", null, stderr);
    }
    return JSON.parse(stdout) as ScanResultVO;
  }

  static create(): CliScannerAdapter {
    return new CliScannerAdapter();
  }
}
