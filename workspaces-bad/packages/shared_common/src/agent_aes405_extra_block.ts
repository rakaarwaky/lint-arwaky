// PURPOSE: AES405 — agent that carries block markers beyond Block 3
//
// Fixture for `check_agent_block_markers`: the structure is Block 1 (types and
// injected deps) -> Block 2 (aggregate impl) -> Block 3 (constructors, std
// traits, helpers). Block 4 and Block 5 mean the file has outgrown that shape
// and the behaviour they hold belongs in a capability or utility.
import { FileEntry } from "../../shared/src/filesystem/taxonomy_filesystem_vo";

export interface IFileScanProtocol {
  scan(files: FileEntry[]): FileEntry[];
}

export interface IExtraBlockAggregate {
  execute(files: FileEntry[]): number;
}

// ─── Block 1: Struct Definitions ───────────────────
export class ExtraBlockAgent implements IExtraBlockAggregate {
  constructor(private scanner: IFileScanProtocol) {}

  execute(files: FileEntry[]): number {
    return this.scanner.scan(files).length;
  }
}

// ─── Block 2: Aggregate Trait Implementation ───────
// AES405: no second aggregate block — one aggregate impl belongs in Block 2.
export class ExtraBlockAgent {
  collect(files: FileEntry[]): FileEntry[] {
    return files;
  }
}

// ─── Block 3: Constructors, Std Traits, Helpers ────
export class ExtraBlockAgent {
  normalise(files: FileEntry[]): FileEntry[] {
    return files;
  }
}

// ─── Block 4: Extra Seams ───────────────────
// AES405: no fourth block — fold this into Block 3 or move it out.
export class ExtraBlockAgent {
  summarise(): string {
    return "";
  }
}

// ─── Block 5: Reporting ───────────────────
// AES405: no fifth block either.
export class ExtraBlockAgent {
  report(): string {
    return "";
  }
}