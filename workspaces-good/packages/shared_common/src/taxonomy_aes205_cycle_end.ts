// AES205: Part of circular dependency chain
import { CycleStartEntity } from './taxonomy_aes205_cycle_start';

export class CycleEndEntity {
    data: string = "end";
    process(): string {
        const start = new CycleStartEntity();
        return this.data;
    }
}
