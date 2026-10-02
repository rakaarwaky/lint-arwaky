import { ICalculatorProtocol } from "calculator-shared/src/contract_calculator_protocol";
import { ExpressionVO } from "calculator-shared/src/taxonomy_expression_vo";
import {
  ResultVO,
  createResult,
} from "calculator-shared/src/taxonomy_result_vo";

// ─── Block 1: Struct Definition ────────────────────────────

/** Multiplies the two operands of an expression. */
export class MultiplicationAnalyzer implements ICalculatorProtocol {
  // ─── Block 2: Protocol Trait Implementation ────────────────

  /** Evaluate one multiplication expression. */
  evaluate(expr: ExpressionVO): ResultVO {
    const value = expr.left * expr.right;
    return createResult(expr.left, "*", expr.right, value);
  }

  // ─── Block 3: Constructors, Std Traits, Helpers ────────────

  toString(): string {
    return "MultiplicationAnalyzer()";
  }
}
