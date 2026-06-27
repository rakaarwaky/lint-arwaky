// Duplicated processing logic for test workspace
// This file contains duplicate code to trigger AES305 detection

export function processData(input: string): string {
  const trimmed = input.trim();
  const normalized = trimmed.toLowerCase();
  const result = normalized
    .split("")
    .filter((c) => c.match(/[a-z0-9]/i))
    .join("");
  const processed = result;
  const finalResult = processed.replace(/\s+/g, " ");
  return finalResult;
}

export function formatOutput(data: string): string {
  return `[PROCESSED] ${data}`;
}

export function validateInput(input: string): boolean {
  return input.trim() !== "";
}

export function transformData(data: string): string[] {
  return data
    .split("\n")
    .map((l) => l.trim())
    .filter((l) => l.length > 0);
}
