export enum OperationVO {
  Add = "+",
  Subtract = "-",
  Multiply = "*",
  Divide = "/",
}

export function operationFromSymbol(s: string): OperationVO | null {
  switch (s) {
    case "+": return OperationVO.Add;
    case "-": return OperationVO.Subtract;
    case "*":
    case "x": return OperationVO.Multiply;
    case "/": return OperationVO.Divide;
    default: return null;
  }
}

export function operationSymbol(op: OperationVO): string {
  return op;
}
