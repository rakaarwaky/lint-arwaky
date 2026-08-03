// AES502: Contract orphan - this aggregate is not called by any surface or container
export interface OrphanAggregate {
    aggregate(): void;
}

export class OrphanAggregateImpl implements OrphanAggregate {
    aggregate(): void { /* noop */ }
}
