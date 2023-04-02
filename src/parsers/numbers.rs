use regex::Regex;

pub fn extract_numeric_values<T>(input: T) -> Vec<String>
    where T: ToString
{
    let re = Regex::new(r"\d+").unwrap();
    let mut numeric_values = Vec::new();

    for cap in re.captures_iter(input.to_string().as_str()) {
        let value = cap.get(0).unwrap().as_str().to_owned();
        numeric_values.push(value);
    }

    numeric_values
}