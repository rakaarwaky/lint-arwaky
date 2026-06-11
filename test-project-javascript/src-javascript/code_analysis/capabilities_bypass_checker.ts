// PURPOSE: Test AES022 — bypass comments in JS/TS
// eslint-disable-next-line no-console
// @ts-ignore
const bad: any = "violation";

// @ts-expect-error
const another: any = 123;

function checkSomething(): string {
  // eslint-disable
  return bad;
}
