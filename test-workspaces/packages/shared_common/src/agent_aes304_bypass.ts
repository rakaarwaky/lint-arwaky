// AES304: bypass annotation - any type and type assertions
import { DomainModelBase } from './domain_model_base';

export class BypassEntity {
    unsafe_method(): any {
        const result: any = null;  // any type bypass
        return result;
    }
}
