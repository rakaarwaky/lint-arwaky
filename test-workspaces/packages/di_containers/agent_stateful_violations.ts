// PURPOSE: Test AES0305 — agent non-stateless, any type, infra imports, single goal
// --- Imports violating AES0305 ---

export class AgentStatefulViolations {
  private state: any = null;

  public run(data: any): any {
    // AES0305: any type
    this.state = data; // AES0305: state assignment outside constructor
    return this.process(data);
  }

  private process(data: any): any {
    return path.basename(data); // AES0305: infra import usage
  }
}

export class AgentSingleGoal {
  public execute(): void {
    // AES0305: single execution goal
  }
}
