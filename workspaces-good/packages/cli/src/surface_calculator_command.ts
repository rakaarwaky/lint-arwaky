import { CalculatorAggregate } from "calculator-shared/src/contract_calculator_aggregate";
import { ExpressionVO, createExpression } from "calculator-shared/src/taxonomy_expression_vo";
import { OperationVO, operationFromSymbol } from "calculator-shared/src/taxonomy_operation_vo";
import * as readline from "readline";

export function run(calc: CalculatorAggregate): void {
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stderr,
  });
  console.error("=== Calculator ===");
  console.error("Ketik operasi: <angka> <operator> <angka>");
  console.error("Contoh: 2 + 3");
  console.error("Ketik 'h' untuk riwayat, 'q' untuk keluar");

  const prompt = () => {
    rl.question("> ", (input) => {
      const trimmed = input.trim();
      if (trimmed === "q") {
        console.error("Sampai jumpa!");
        rl.close();
        return;
      }
      if (trimmed === "h") {
        const hist = calc.history();
        if (hist.length === 0) {
          console.error("  Belum ada riwayat");
        } else {
          hist.forEach((r) => console.error(`  ${r.expression}`));
        }
        prompt();
        return;
      }
      const parts = trimmed.split(/\s+/);
      if (parts.length !== 3) {
        console.error("  Format: <angka> <operator> <angka>");
        prompt();
        return;
      }
      const left = parseFloat(parts[0]);
      const op = operationFromSymbol(parts[1]);
      const right = parseFloat(parts[2]);
      if (isNaN(left) || isNaN(right)) {
        console.error("  Input bukan angka");
        prompt();
        return;
      }
      if (op === null) {
        console.error(`  '${parts[1]}' bukan operator valid`);
        prompt();
        return;
      }
      const expr: ExpressionVO = createExpression(left, op, right);
      const result = calc.delegate(expr);
      if (result) {
        console.error(`  = ${result.value}`);
      } else {
        console.error("  Error: tidak bisa hitung");
      }
      prompt();
    });
  };
  prompt();
}
