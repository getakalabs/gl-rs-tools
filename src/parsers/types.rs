use calamine::DataType;

pub fn string(value: &Option<&DataType>) -> Option<String> {
    match *value {
        None => None,
        Some(data) => {
            match data {
                DataType::String(s) => Some(s.to_string()),
                DataType::Float(f) => Some(f.to_string()),
                DataType::Int(i) => Some(i.to_string()),
                DataType::Bool(b) => Some(b.to_string()),
                _ => None,
            }
        }
    }
}

pub fn int(value: &Option<&DataType>)-> Option<i32> {
    match *value {
        None => None,
        Some(data) => {
            match data {
                DataType::String(s) => {
                    match s.parse::<i32>() {
                        Ok(i) => Some(i),
                        Err(_) => None,
                    }
                },
                DataType::Float(f) => Some(*f as i32),
                DataType::Int(i) => Some(*i as i32),
                _ => None,
            }
        }
    }
}
