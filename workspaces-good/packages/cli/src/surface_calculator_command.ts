import { ICalculatorAggregate } from "calculator-shared/src/contract_calculator_aggregate";
import {
  requestDelegate,
  requestHistory,
} from "calculator-shared/src/taxonomy_calculator_request_vo";
import { parseExpression } from "calculator-shared/src/utility_expression_parser";
import * as readline from "readline";

export function run(calc: ICalculatorAggregate): void {
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
        const resp = calc.execute(requestHistory());
        if (resp.kind === "history") {
          if (resp.results.length === 0) {
            console.error("  Belum ada riwayat");
          } else {
            resp.results.forEach((r) => console.error(`  ${r.expression}`));
          }
        }
        prompt();
        return;
      }
      const expr = parseExpression(trimmed);
      if (expr === null) {
        console.error("  Format: <angka> <operator> <angka>");
        prompt();
        return;
      }
      const resp = calc.execute(requestDelegate(expr));
      if (resp.kind === "delegation" && resp.result) {
        console.error(`  = ${resp.result.value}`);
      } else {
        console.error("  Error: tidak bisa hitung");
      }
      prompt();
    });
  };
  prompt();
}
