// Highly violating file designed to maximize errors
import { NonExistentService } from "../nonexistent/service";
import { AnotherFakeSymbol } from "../../capabilities/fake/symbol";

// eslint-disable-next-line
// @ts-ignore
const violatingAnyVar: any = "illegal assignment to any";
const wrongTypeNumber: number = "this is definitely not a number";
const anotherWrongType: boolean = 12345;

debugger;

export class BadHollowClass extends NonExistentService {
  public execute(): any {
    // eslint-disable
    const x: any = violatingAnyVar;
    return x;
  }
}
