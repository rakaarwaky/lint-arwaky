// AES203: capabilities with unused import
import { DomainModelBase } from './domain_model_base';

export class UnusedImportChecker {
    check(): boolean {
        // DomainModelBase is imported but never used
        return true;
    }
}
