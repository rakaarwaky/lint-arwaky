import { ICalculatorProtocol } from "calculator-shared/src/contract_calculator_protocol";
import { ExpressionVO } from "calculator-shared/src/taxonomy_expression_vo";
import {
  ResultVO,
  createResult,
} from "calculator-shared/src/taxonomy_result_vo";

// ─── Block 1: Struct Definition ────────────────────────────

/** Subtracts the right operand of an expression from the left one. */
export class SubtractionAnalyzer implements ICalculatorProtocol {
  // ─── Block 2: Protocol Trait Implementation ────────────────

  /** Evaluate one subtraction expression. */
  evaluate(expr: ExpressionVO): ResultVO {
    const value = expr.left - expr.right;
    return createResult(expr.left, "-", expr.right, value);
  }

  // ─── Block 3: Constructors, Std Traits, Helpers ────────────

  toString(): string {
    return "SubtractionAnalyzer()";
  }
}
