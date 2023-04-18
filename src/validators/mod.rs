// ToDo: Clean up the entire validator module
use regex::Regex;
use validator::validate_email as validator_email;

use crate::traits::GetI32;
use crate::traits::IsEmpty;
use crate::Cipher;
use crate::Primitive;

pub fn validate_email<T>(value: &Option<T>) -> Option<&str>
    where T: ToString
{
    match value {
        None => Some("empty"),
        Some(value) => {
            let value = value.to_string();

            match value.trim().is_empty() {
                true => return Some("empty"),
                false => if !validator_email(value) {
                    return Some("invalid");
                },
            }

            None
        }
    }
}

pub fn validate_name<T>(value: &Option<T>, min: Option<usize>) -> Option<&str>
    where T: ToString
{
    match value {
        None => Some("empty"),
        Some(value) => {
            let value = value.to_string();

            match value.trim().is_empty() {
                true => return Some("empty"),
                false => if let Some(min) = min {
                    if value.len() < min {
                        return Some("invalid");
                    } else {
                        let regex = Regex::new(r#"^[\p{L} ,.`'-]*$"#).unwrap();
                        let cap = regex.captures(value.as_str());
                        if cap.is_none() {
                            return Some("invalid");
                        }
                    }
                }
            }

            None
        }
    }
}

pub fn validate_password<E, I>(input: &Option<Cipher>, strict: bool, min: usize, max: usize, empty: E, invalid: I) -> Option<Cipher>
    where E: ToString,
          I: ToString
{
    let empty = empty.to_string();
    let invalid = invalid.to_string();

    let error_min = format!("minimum of {} characters allowed", min.clone());
    let error_max = format!("maximum of {} characters allowed", max.clone());
    let error_number = String::from("at least 1 number");
    let error_uppercase = String::from("at least 1 uppercase character");
    let error_lowercase = String::from("at least 1 lowercase character");
    let error_special_character = String::from("at least 1 special character");

    match strict {
        true => {
            let mut password = Cipher::payload();

            match input.clone() {
                None => {
                    password.minimum = Some(error_min);
                    password.uppercase = Some(error_uppercase);
                    password.lowercase = Some(error_lowercase);
                    password.number = Some(error_number);
                    password.special_character = Some(error_special_character);
                }
                Some(value) => {
                    let value = value.to_string();

                    if value.len() < min {
                        password.minimum = Some(error_min);
                    }

                    if value.len() > max {
                        password.maximum = Some(error_max);
                    }

                    if !crate::strings::has_uppercase(&value) {
                        password.uppercase = Some(error_uppercase);
                    }

                    if !crate::strings::has_lowercase(&value) {
                        password.lowercase = Some(error_lowercase);
                    }

                    if crate::strings::is_alphanumeric(&value) {
                        password.special_character = Some(error_special_character);
                    }

                    if crate::strings::is_alphabetic(&value) {
                        password.number = Some(error_number);
                    }
                }
            };

            match password.is_empty() {
                true => None::<Cipher>,
                false => Some(Cipher::from(password))
            }
        }
        false => {
            match input.clone() {
                None => Some(Cipher::String(empty)),
                Some(value) => {
                    let value = value.to_string();

                    match value.len() < min {
                        true => Some(Cipher::String(invalid)),
                        false => None::<Cipher>
                    }
                }
            }
        }
    }
}

pub fn validate_primitive_i32(value: &Option<Primitive>, min: Option<usize>) -> Option<&str> {
    match value {
        None => Some("empty"),
        Some(value) => {
            match value.get_i32() {
                None => Some("empty"),
                Some(value) => {
                    if let Some(min) = min {
                        if value < min as i32 {
                            return Some("invalid");
                        }
                    }

                    None
                }
            }
        }
    }
}

pub fn validate_string<T>(value: &Option<T>, min: Option<usize>) -> Option<&str>
    where T: ToString
{
    match value {
        None => Some("empty"),
        Some(value) => {
            let value = value.to_string();

            match value.trim().is_empty() {
                true => return Some("empty"),
                false => if let Some(min) = min {
                    if value.len() < min {
                        return Some("invalid");
                    }
                }
            }

            None
        }
    }
}

pub fn validate_string_base64<T>(value: &Option<T>, min: Option<usize>) -> Option<&str>
    where T: ToString
{
    match value {
        None => Some("empty"),
        Some(value) => {
            let value = value.to_string();

            match value.trim().is_empty() {
                true => return Some("empty"),
                false => {
                    match base64_url::decode(&value) {
                        Ok(value) => {
                            if let Some(min) = min {
                                if value.len() < min {
                                    return Some("invalid");
                                }
                            }
                        },
                        Err(_) => return Some("invalid"),
                    }
                }
            }

            None
        }
    }
}

pub fn validate_string_options<'a, T, U>(value: &'a Option<T>, options: &'a [U]) -> Option<&'a str>
    where T: ToString,
          U: ToString
{
    match value {
        None => Some("empty"),
        Some(value) => {
            let value = value.to_string();

            match value.trim().is_empty() {
                true => return Some("empty"),
                false => {
                    let mut found = false;
                    for option in options {
                        if value.to_lowercase() == option.to_string().to_lowercase() {
                            found = true;
                            break;
                        }
                    }

                    if !found {
                        return Some("invalid");
                    }
                }
            }

            None
        }
    }
}


