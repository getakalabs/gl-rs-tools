use regex::Regex;
use validator::validate_email as validator_email;

use crate::Cipher;
use crate::Primitive;

use crate::traits::prelude::*;

pub fn validate_email<E, I>(value: &Option<String>, empty: E, invalid: I) -> Option<String>
    where E: ToString,
          I: ToString
{
    let empty = empty.to_string();
    let invalid = invalid.to_string();

    value.clone().map_or(Some(empty.clone()), |data| {
        match () {
            _ if data.is_empty() => Some(empty),
            _ if !validator_email(data) => Some(invalid),
            _ => None
        }
    })
}

pub fn validate_name<E, I>(input: &Option<String>, empty: E, invalid: I, min: Option<usize>) -> Option<String>
    where E: ToString,
          I: ToString
{
    let empty = empty.to_string();
    let invalid = invalid.to_string();

    let value = input.clone().unwrap_or(String::default()).to_lowercase();
    if value.is_empty() {
        return Some(empty);
    }

    if min.is_some() && value.len() < min.unwrap() {
        return Some(invalid);
    }

    let regex = Regex::new(r#"^[\p{L} ,.`'-]*$"#).unwrap();
    let cap = regex.captures(value.as_str());
    if cap.is_none() {
        return Some(invalid);
    }

    None
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
            let mut password = Cipher::default_payload();

            match input.clone() {
                None => {
                    password.minimum = Some(error_min);
                    password.uppercase = Some(error_uppercase);
                    password.lowercase = Some(error_lowercase);
                    password.number = Some(error_number);
                    password.special_character = Some(error_special_character);
                }
                Some(value) => {
                    let value = value.to_string().unwrap_or(String::default());

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
                false => Cipher::new_payload(&password)
            }
        }
        false => {
            match input.clone() {
                None => Cipher::new_string(empty),
                Some(value) => {
                    let value = value.to_string().unwrap_or(String::default());

                    match value.len() < min {
                        true => Cipher::new_string(invalid),
                        false => None::<Cipher>
                    }
                }
            }
        }
    }
}

pub fn validate_primitive_i32<E, I>(value: &Option<Primitive>, empty: E, invalid: I, min: Option<usize>) -> Option<Primitive>
    where E: ToString,
          I: ToString
{
    let empty = empty.to_string();
    let invalid = invalid.to_string();

    match value.clone() {
        None => Some(Primitive::String(empty)),
        Some(data) => {
            match data.get_i32() {
                None => Some(Primitive::String(empty)),
                Some(data) => {
                    if min.is_some() && data < min.unwrap() as i32 {
                        return Some(Primitive::String(invalid))
                    }

                    None
                }
            }
        }
    }
}


pub fn validate_string<E, I>(value: &Option<String>, empty: E, invalid: I, min: Option<usize>) -> Option<String>
    where E: ToString,
          I: ToString
{
    let empty = empty.to_string();
    let invalid = invalid.to_string();

    value.clone().map_or(Some(empty.clone()), |data| {
        match () {
            _ if data.is_empty() => Some(empty),
            _ if min.is_some() && data.len() < min.unwrap() => Some(invalid),
            _ => None
        }
    })
}

pub fn validate_string_base64<E, I>(value: &Option<String>, empty: E, invalid: I, min: usize) -> Option<String>
    where E: ToString,
          I: ToString
{
    let empty = empty.to_string();
    let invalid = invalid.to_string();

    value.clone().map_or(Some(empty), |data| {
        let decoded = base64_url::decode(&data);
        if decoded.is_err() {
            return Some(invalid.clone());
        }

        let decoded = &decoded.unwrap()[..];
        if decoded.len() < min {
            return Some(invalid.clone());
        }

        None
    })
}

pub fn validate_string_options<T, E, I>(value: &Option<T>, empty: E, invalid: I, options: &[T]) -> Option<T>
    where T: ToString + Clone + From<String> + PartialEq,
          E: ToString,
          I: ToString
{
    let empty = empty.to_string();
    let invalid = invalid.to_string();

    match value.clone() {
        None => Some(T::from(empty)),
        Some(data) => {
            let value = data.to_string().to_lowercase();
            let mut opts:Vec<String> = Vec::new();

            for i in options {
                let v = i.to_string().to_lowercase();
                if !v.is_empty() {
                    opts.push(v);
                }
            }

            match !opts.contains(&value) {
                true => Some(T::from(invalid)),
                false => None
            }
        }
    }
}

