/** Utility: Stateless PATH resolver — finds executables in PATH.

I/O allowed: domain-agnostic + reusable across modules.
*/

import * as fs from "node:fs";
import * as path from "node:path";

const isWindows = process.platform === "win32";

/**
 * Resolve an executable name to its absolute path by searching PATH.
 * Returns null when the executable is not found.
 */
export function whichSync(
  name: string,
): string | null {
  if (path.isAbsolute(name)) {
    return fs.existsSync(name) ? name : null;
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

  return null;
}
