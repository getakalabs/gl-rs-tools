/// Retrieves the multiplier value to return it to float
pub fn get_multiplier(value: f64) -> i64 {
    let num_decimals = value.to_string().split('.').nth(1).unwrap_or("").len();
    10_i64.pow(num_decimals as u32)
}

/// Iterates to list of numbers
pub fn iter_numbers(numbers: Vec<i32>, limit: i32, current: &mut String) -> String {
    // Return current if less than 1
    if limit < 1 {
        return current.clone();
    }

    // Create edge
    let edge = match limit > 9 {
        true => 9,
        false => limit
    };

    // Create extra
    let extra = match limit > 9 {
        true => limit - 9,
        false => 0
    };

    let nums: String = numbers[0..=( (edge - 1) as usize)].iter().map( |&id| id.to_string()).collect();
    let mut current = format!("{current}{nums}");

    iter_numbers(numbers, extra, &mut current)
}