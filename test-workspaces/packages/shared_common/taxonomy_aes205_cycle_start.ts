// AES205: Part of circular dependency chain
import { CycleEndEntity } from './taxonomy_aes205_cycle_end';

export class CycleStartEntity {
    data: string = "start";
    process(): string {
        const end = new CycleEndEntity();
        return this.data;
    }
}
