import { ExpressionVO, createExpression } from "./taxonomy_expression_vo";
import { operationFromSymbol } from "./taxonomy_operation_vo";

export function parseOperand(input: string): number | null {
  const n = parseFloat(input.trim());
  return isNaN(n) ? null : n;
}

export function parseExpression(input: string): ExpressionVO | null {
  const parts = input.trim().split(/\s+/);
  if (parts.length !== 3) return null;
  const left = parseOperand(parts[0]);
  const op = operationFromSymbol(parts[1]);
  const right = parseOperand(parts[2]);
  if (left === null || right === null || op === null) return null;
  return createExpression(left, op, right);
}
