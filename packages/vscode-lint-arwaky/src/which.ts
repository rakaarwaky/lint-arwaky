import * as fs from "node:fs";
import * as path from "node:path";

const isWindows = process.platform === "win32";

export function sync(name: string, options?: { nothrow?: boolean }): string | null {
  if (path.isAbsolute(name)) {
    if (fs.existsSync(name)) {
      return name;
    }
    if (!options?.nothrow) {
      throw new Error(`Not found: ${name}`);
    }
    return null;
  }

  const pathDirs = (process.env.PATH ?? "").split(isWindows ? ";" : ":");
  const extensions = isWindows ? [".exe", ".cmd", ".bat", ""] : [""];

  for (const dir of pathDirs) {
    for (const ext of extensions) {
      const full = path.join(dir, name + ext);
      if (fs.existsSync(full) && fs.statSync(full).isFile()) {
        return full;
      }
    }
  }

  if (!options?.nothrow) {
    throw new Error(`Not found in PATH: ${name}`);
  }
  return null;
}
