// PURPOSE: Test AES0306 — surface passive with active logic, hierarchy violation, >15 functions
// Required taxonomy import for surface layer
import { DescriptionVO } from "../taxonomy/description_vo";

export class SurfacePassiveView {
  // Passive component with >15 functions = AES0306
}

function helper1(): void {}
function helper2(): void {}
function helper3(): void {}
function helper4(): void {}
function helper5(): void {}
function helper6(): void {}
function helper7(): void {}
function helper8(): void {}
function helper9(): void {}
function helper10(): void {}
function helper11(): void {}
function helper12(): void {}
function helper13(): void {}
function helper14(): void {}
function helper15(): void {}
function helper16(): void {} // >15 functions = AES0306
function helper17(): void {}
function helper18(): void {}

export class SurfaceWithDomainLogic {
  public businessLogic(data: number[]): number[] {
    // AES0306: passive surface with active domain logic (deep nesting)
    const result: number[] = [];
    for (const item of data) {
      if (item > 0) {
        if (item % 2 === 0) {
          if (item > 10) {
            result.push(item * 2);
          } else {
            result.push(item);
          }
        } else {
          result.push(item * 3);
        }
      } else {
        result.push(0);
      }
    }
    return result;
  }
}
