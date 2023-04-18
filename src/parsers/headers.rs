use std::ops::Index;
use anyhow::Result;
use std::str::FromStr;
use actix_web::http::header::HeaderValue;

// Parse range header to get start and end
// Author: Deneir
pub fn get(range_header: &HeaderValue) -> Result<(usize, usize)> {
    let s = range_header.to_str().unwrap();
    let prefix = "bytes=";

    if !s.starts_with(prefix) {
        return Err(anyhow::anyhow!("Range header doesn't start with 'bytes='"));
    }

    let split = (s[prefix.len()..])
        .split('-')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    if split.len() != 2 {
        return Err(anyhow::anyhow!("Range header has an invalid format"));
    }

    let start = usize::from_str(split.index(0))?;
    let end = usize::from_str(split.index(1))?;

    Ok((start, end))
}