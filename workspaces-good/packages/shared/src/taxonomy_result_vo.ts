export interface ResultVO {
  expression: string;
  value: number;
}

export function createResult(
  left: number,
  op: string,
  right: number,
  value: number,
): ResultVO {
  return {
    expression: `${left} ${op} ${right} = ${value}`,
    value,
  };
}
