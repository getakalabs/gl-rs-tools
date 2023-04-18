 use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};

// Converts string to naive date time
pub fn naive_date_time<T>(value: T) -> Option<NaiveDateTime>
    where T: ToString
{
    // Set value
    let value = value.to_string();

    // RFC2822 = Date + Time + TimeZone
    if let Ok(item) = DateTime::parse_from_rfc2822(&value) {
        return Some(item.naive_utc());
    }

    // Parse postgres date + time
    if let Ok(item) = DateTime::parse_from_str(&format!("{}+00", &value), "%Y-%m-%dT%H:%M:%S%.f%#z") {
        return Some(item.with_timezone(&Utc).naive_utc());
    }

    // RFC3339 = Date + Time + TimeZone
    if let Ok(item) = DateTime::parse_from_rfc3339(&value) {
        return Some(item.with_timezone(&Utc).naive_utc());
    };

    // Date + Time + Timezone (other or non-standard)
    if let Ok(item) = DateTime::parse_from_str(&value, "%Y-%m-%d %H:%M:%S %z") {
        return Some(item.naive_utc());
    }

    // Date + Time only
    if let Ok(item) = NaiveDateTime::parse_from_str(&value, "%Y-%m-%d %H:%M:%S") {
        return Some(item);
    }

    if let Ok(item) = NaiveDateTime::parse_from_str(&value, "%Y-%m-%d %H:%M:%S") {
        return Some(item);
    }

    // If none return None
    None
}

/// Converts string to naive date
pub fn naive_date<T>(value: T) -> Option<NaiveDate>
    where T: ToString
{
    // Set value
    let value = value.to_string();

    // RFC2822 = Date + Time + TimeZone
    if let Ok(item) = DateTime::parse_from_rfc2822(&value) {
        return Some(item.naive_utc().date())
    }

    // Parse postgres date + time
    if let Ok(item) = DateTime::parse_from_str(&format!("{}+00", &value), "%Y-%m-%dT%H:%M:%S%.f%#z") {
        return Some(item.with_timezone(&Utc).naive_utc().date())
    }

    // RFC3339 = Date + Time + TimeZone
    if let Ok(item) = DateTime::parse_from_rfc3339(&value) {
        return Some(item.with_timezone(&Utc).naive_utc().date())
    }

    // Date + Time + Timezone (other or non-standard)
    if let Ok(item) = DateTime::parse_from_str(&value, "%Y-%m-%d %H:%M:%S %z") {
        return Some(item.naive_utc().date())
    }

    if let Ok(item) = DateTime::parse_from_str(&value, "%Y-%m-%d %H:%M:%S %z") {
        return Some(item.naive_utc().date())
    };

    // Date + Time only
    match  NaiveDateTime::parse_from_str(&value, "%Y-%m-%d %H:%M:%S") {
        Ok(item) => Some(item.date()),
        Err(_) => None
    }
}