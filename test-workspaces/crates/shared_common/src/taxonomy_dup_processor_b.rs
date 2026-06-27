// Duplicated processing logic for test workspace
// This file contains duplicate code to trigger AES305 detection

pub fn process_data(input: &str) -> String {
    let trimmed = input.trim();
    let normalized = trimmed.to_lowercase();
    let result: Vec<char> = normalized.chars().filter(|c| c.is_alphanumeric()).collect();
    let processed = result.into_iter().collect::<String>();
    let final_result = processed.replace("  ", " ");
    final_result
}

pub fn format_output(data: &str) -> String {
    format!("[PROCESSED] {}", data)
}

pub fn validate_input(input: &str) -> bool {
    !input.trim().is_empty()
}

pub fn transform_data(data: &str) -> Vec<String> {
    data.lines().map(|l| l.trim().to_string()).filter(|l| !l.is_empty()).collect()
}
