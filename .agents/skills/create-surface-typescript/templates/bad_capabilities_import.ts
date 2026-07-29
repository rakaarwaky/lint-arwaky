// BAD: Smart surface imports capabilities directly
import { MyChecker } from '../capabilities/my_checker';

export class CheckCommand {
    constructor() {
        this._checker = new MyChecker();
    }
}
