use std::str::FromStr;
use actix_web::http::header::HeaderValue;

use crate::Errors;

// Parse range header to get start and end
// Author: Deneir
pub fn get(range_header: &HeaderValue) -> Result<(usize, usize), Errors> {
    let s = range_header.to_str().unwrap();
    let prefix = "bytes=";

    if !s.starts_with(prefix) {
        return Err(Errors::new("Range header doesn't start with 'bytes='"));
    }

    let split = (s[prefix.len()..])
        .split('-')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    if split.len() != 2 {
        return Err(Errors::new("Range header doesn't have exactly two parts"));
    }

    let start = match usize::from_str(&split[0]) {
        Ok(s) => s,
        Err(_) => return Err(Errors::new("Range header has an invalid start index")),
    };

    match usize::from_str(&split[1]) {
        Ok(end) => Ok((start, end)),
        Err(_) => Err(Errors::new("Range header has an invalid end index")),
    }
}