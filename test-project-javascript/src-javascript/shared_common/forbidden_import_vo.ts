use crate::capabilities::some_module;
use crate::infrastructure::some_adapter;

export class ForbiddenImportVo {
    value: string;
    constructor(value: string) {
        this.value = value;
    }
}
