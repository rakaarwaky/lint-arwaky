// AES404: utility defines an enum — utility is free functions only.
export enum Mode {
  Fast,
  Slow,
}

export function pick(mode: Mode): string {
  return mode === Mode.Fast ? "fast" : "slow";
}
