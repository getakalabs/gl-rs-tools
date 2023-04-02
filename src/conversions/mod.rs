pub fn to_string_array<T: ToString>(value: &Option<Vec<T>>) -> Option<Vec<String>> {
    let mut array:Vec<String> = Vec::new();

    match value {
        Some(value) => {
            for item in value {
                let i = item.to_string();
                if !i.is_empty() {
                    array.push(item.to_string());
                }
            }

            match array.is_empty() {
                true => None,
                false => Some(array)
            }
        },
        None => None
    }
}