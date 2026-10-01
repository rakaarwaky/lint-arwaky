// PURPOSE: AES405 P14 — agent that implements contract protocols itself
//
// Fixture for `check_agent_single_aggregate`: an agent is the feature's
// composition root — it implements the feature aggregate and injects protocol
// seams. Implementing a protocol here makes the orchestration layer duplicate a
// capability's work, so both implementations below are violations. Each belongs
// in a `capabilities_*` file that the agent then delegates to.
import { FileEntry } from "../../shared/src/filesystem/taxonomy_filesystem_vo";

export interface IFileScanProtocol {
  scan(files: FileEntry[]): FileEntry[];
}

export interface IReportProtocol {
  report(): string;
}

export interface IProtocolImplAggregate {
  execute(files: FileEntry[]): number;
}

export class ProtocolImplAgent implements IProtocolImplAggregate {
  constructor(
    private scanner: IFileScanProtocol,
    private reporter: IReportProtocol,
  ) {}

  execute(files: FileEntry[]): number {
    return this.scanner.scan(files).length;
  }
}

// AES405: belongs in `capabilities_file_scanner`, not in the agent.
export class ProtocolImplAgent implements IFileScanProtocol {
  scan(files: FileEntry[]): FileEntry[] {
    return files;
  }
}

// AES405: belongs in `capabilities_file_reporter`, not in the agent.
export class ProtocolImplAgent implements IReportProtocol {
  report(): string {
    return "";
  }
}