use crate::surfaces::cli_main_entry::CliMainEntry;
use crate::taxonomy::order_entity::OrderEntity;

export class BadHookStore {
    private entry: CliMainEntry;

    constructor(entry: CliMainEntry) {
        this.entry = entry;
    }

    getData(): OrderEntity {
        return this.entry.fetchData();
    }
}
