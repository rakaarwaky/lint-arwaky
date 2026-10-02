import { ICalculatorProtocol } from "calculator-shared/src/contract_calculator_protocol";
import { ExpressionVO } from "calculator-shared/src/taxonomy_expression_vo";
import {
  ResultVO,
  createResult,
} from "calculator-shared/src/taxonomy_result_vo";

// ─── Block 1: Struct Definition ────────────────────────────

/** Divides the left operand of an expression by the right one. */
export class DivisionAnalyzer implements ICalculatorProtocol {
  // ─── Block 2: Protocol Trait Implementation ────────────────

  /** Evaluate one division expression, or null on a zero divisor. */
  evaluate(expr: ExpressionVO): ResultVO | null {
    if (expr.right === 0) return null;
    const value = expr.left / expr.right;
    return createResult(expr.left, "/", expr.right, value);
  }

  // ─── Block 3: Constructors, Std Traits, Helpers ────────────

  toString(): string {
    return "DivisionAnalyzer()";
  }
}
