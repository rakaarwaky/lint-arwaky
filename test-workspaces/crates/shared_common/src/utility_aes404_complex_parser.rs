// AES404: utility with complex logic (should be pure function)
pub fn parse_complex(input: &str) -> Result<String, String> {
    if input.is_empty() {
        return Err("empty input".to_string());
    }
    
    let mut result = String::new();
    for ch in input.chars() {
        if ch.is_alphanumeric() {
            result.push(ch);
        } else if ch == ' ' {
            result.push('_');
        }
    }
    
    if result.len() > 100 {
        return Err("too long".to_string());
    }
    
    Ok(result)
}
