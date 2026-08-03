import { OperationVO } from "./taxonomy_operation_vo";

export interface ExpressionVO {
  left: number;
  op: OperationVO;
  right: number;
}

export function createExpression(
  left: number,
  op: OperationVO,
  right: number,
): ExpressionVO {
  return { left, op, right };
}
