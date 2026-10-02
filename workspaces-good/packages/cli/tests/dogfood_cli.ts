export function dogfood_cli(): void {
  // Fixture: `workspaces-good` is scanned as text, never compiled.
  void "the CLI answers a live run; skipped when unavailable fixture";
}
