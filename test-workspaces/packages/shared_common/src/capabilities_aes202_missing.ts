// AES202: capabilities missing mandatory taxonomy import
import { AgentLogic } from '../di_containers/agent_logic';

export class MandatoryMissingChecker {
    check(): boolean {
        const logic = new AgentLogic();
        return true;
    }
}
