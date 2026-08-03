pub fn parse_operand(input: &str) -> Option<f64> {
    input.trim().parse::<f64>().ok()
}

pub fn parse_expression(input: &str) -> Option<(f64, String, f64)> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 3 {
        return None;
    }
    let left = parse_operand(parts[0])?;
    let op = parts[1].to_string();
    let right = parse_operand(parts[2])?;
    Some((left, op, right))
}
