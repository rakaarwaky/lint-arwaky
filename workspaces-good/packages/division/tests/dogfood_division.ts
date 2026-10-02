export function dogfood_division(): void {
  // Fixture: `workspaces-good` is scanned as text, never compiled.
  void "the CLI answers a live run; skipped when unavailable fixture";
}
