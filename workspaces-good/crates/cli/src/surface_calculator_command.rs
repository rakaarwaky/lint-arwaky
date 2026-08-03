use calculator_shared::contract_calculator_aggregate::CalculatorAggregate;
use calculator_shared::taxonomy_expression_vo::ExpressionVO;
use calculator_shared::taxonomy_operation_vo::OperationVO;

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
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 3 {
            eprintln!("  Format: <angka> <operator> <angka>");
            continue;
        }
        let left: f64 = match parts[0].parse() {
            Ok(v) => v,
            Err(_) => {
                eprintln!("  '{}' bukan angka", parts[0]);
                continue;
            }
        };
        let op = match OperationVO::from_symbol(parts[1]) {
            Some(v) => v,
            None => {
                eprintln!("  '{}' bukan operator valid", parts[1]);
                continue;
            }
        };
        let right: f64 = match parts[2].parse() {
            Ok(v) => v,
            Err(_) => {
                eprintln!("  '{}' bukan angka", parts[2]);
                continue;
            }
        };
        let expr = ExpressionVO::new(left, op, right);
        match calc.delegate(&expr) {
            Some(r) => eprintln!("  = {}", r.value),
            None => eprintln!("  Error: tidak bisa hitung"),
        }
    }
    eprintln!("Sampai jumpa!");
}
