use calculator_shared::contract_calculator_aggregate::CalculatorAggregate;
use calculator_shared::taxonomy_expression_vo::ExpressionVO;
use calculator_shared::taxonomy_operation_vo::OperationVO;
use calculator_shared::utility_expression_parser;

pub fn run(calc: &mut dyn CalculatorAggregate) {
    eprintln!("=== Calculator ===");
    eprintln!("Ketik operasi: <angka> <operator> <angka>");
    eprintln!("Contoh: 2 + 3");
    eprintln!("Ketik 'h' untuk riwayat, 'q' untuk keluar");
    loop {
        eprint!("> ");
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_err() {
            eprintln!("  Gagal membaca input");
            continue;
        }
        let input = input.trim();
        if input == "q" {
            break;
        }
        if input == "h" {
            let history = calc.history();
            if history.is_empty() {
                eprintln!("  Belum ada riwayat");
            } else {
                for r in &history {
                    eprintln!("  {}", r.expression);
                }
            }
            continue;
        }
        if let Some((left, op_str, right)) = utility_expression_parser::parse_expression(input) {
            let op = match OperationVO::from_symbol(&op_str) {
                Some(v) => v,
                None => {
                    eprintln!("  '{}' bukan operator valid", op_str);
                    continue;
                }
            };
            let expr = ExpressionVO::new(left, op, right);
            match calc.delegate(&expr) {
                Some(r) => eprintln!("  = {}", r.value),
                None => eprintln!("  Error: tidak bisa hitung"),
            }
        } else {
            eprintln!("  Format: <angka> <operator> <angka>");
        }
    }
    eprintln!("Sampai jumpa!");
}
