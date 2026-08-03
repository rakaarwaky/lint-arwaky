import * as vscode from "vscode";
import { execFile } from "node:child_process";
import { promisify } from "node:util";
import * as which from "./which";
import type { ScanResult } from "./types";

const execFileAsync = promisify(execFile);

export class CliError extends Error {
  constructor(
    message: string,
    public readonly exitCode: number | null,
    public readonly stderr: string,
  ) {
    super(message);
    this.name = "CliError";
  }
}

export function getCliPath(): string {
  const config = vscode.workspace.getConfiguration("lint-arwaky");
  return config.get<string>("cliPath", "lint-arwaky-cli");
}

export async function ensureCliAvailable(): Promise<string> {
  const cliPath = getCliPath();
  const resolved = which.sync(cliPath, { nothrow: true });

  if (!resolved) {
    const installCmd =
      cliPath === "lint-arwaky-cli"
        ? "cargo install lint-arwaky-cli"
        : `Install ${cliPath} and add to PATH`;

    throw new CliError(
      `lint-arwaky-cli not found in PATH.\n\nInstall it first:\n  ${installCmd}`,
      null,
      "",
    );
  }

  return resolved;
}

export async function scan(
  targetPath: string,
  language?: string,
): Promise<ScanResult> {
  const cliPath = await ensureCliAvailable();

  const args = ["scan", targetPath, "--format", "json"];
  if (language) {
    args.push("--language", language);
  }

  try {
    const { stdout, stderr } = await execFileAsync(cliPath, args, {
      maxBuffer: 10 * 1024 * 1024,
      timeout: 120_000,
    });

    if (stderr) {
      console.warn(`[lint-arwaky] stderr: ${stderr}`);
    }

    const result = JSON.parse(stdout) as ScanResult;
    return result;
  } catch (err: unknown) {
    if (err instanceof Error && "code" in err) {
      const nodeErr = err as { code: number; stderr?: string; message: string };
      throw new CliError(
        `lint-arwaky-cli exited with code ${nodeErr.code}`,
        nodeErr.code,
        nodeErr.stderr ?? "",
      );
    }
    throw err;
  }
}
