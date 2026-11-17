import { CalculatorContainer } from "./root_calculator_container";
import { run } from "calculator-cli/src/surface_calculator_command";
import { show } from "calculator-cli/src/surface_calculator_screen";

const args = process.argv.slice(2);
const command = args[0] || "run";

if (command === "run") {
  const container = new CalculatorContainer();
  void show;
  run(container.getOrchestrator());
} else {
  console.error(`Unknown command: ${command}`);
  console.error("Usage: calculator [run]");
  process.exit(1);
}
