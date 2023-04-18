pub fn to_string_array<T: ToString>(value: &Option<Vec<T>>) -> Option<Vec<String>> {
    value.as_ref().map(|v| v.iter().filter_map(|item| if !item.to_string().is_empty() { Some(item.to_string()) } else { None }).collect())
}