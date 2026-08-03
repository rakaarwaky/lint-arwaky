import sys
from typing import List

from shared.src.contract_calculator_aggregate import CalculatorAggregate
from shared.src.taxonomy_expression_vo import create_expression
from shared.src.taxonomy_operation_vo import operation_from_symbol


def run(calc: CalculatorAggregate) -> None:
    print("=== Calculator ===", file=sys.stderr)
    print("Ketik operasi: <angka> <operator> <angka>", file=sys.stderr)
    print("Contoh: 2 + 3", file=sys.stderr)
    print("Ketik 'h' untuk riwayat, 'q' untuk keluar", file=sys.stderr)

    while True:
        try:
            line = input("> ")
        except EOFError:
            break
        trimmed = line.strip()
        if trimmed == "q":
            break
        if trimmed == "h":
            hist: List = calc.history()
            if not hist:
                print("  Belum ada riwayat", file=sys.stderr)
            else:
                for r in hist:
                    print(f"  {r.expression}", file=sys.stderr)
            continue
        parts = trimmed.split()
        if len(parts) != 3:
            print("  Format: <angka> <operator> <angka>", file=sys.stderr)
            continue
        try:
            left = float(parts[0])
            right = float(parts[2])
        except ValueError:
            print("  Input bukan angka", file=sys.stderr)
            continue
        op = operation_from_symbol(parts[1])
        if op is None:
            print("  Operator tidak dikenal", file=sys.stderr)
            continue
        expr = create_expression(left, op, right)
        result = calc.delegate(expr)
        if result:
            print(f"  = {result.value}", file=sys.stderr)
        else:
            print("  Error: tidak bisa hitung", file=sys.stderr)
    print("Sampai jumpa!", file=sys.stderr)
