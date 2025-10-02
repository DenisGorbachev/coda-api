use std::num::ParseIntError;
use thiserror::Error;
use time::Duration;

pub fn parse_duration_value(source: &impl AsRef<str>) -> Result<Option<Duration>, DurationValueParserError> {
    let trimmed = source.as_ref().trim();
    if trimmed.is_empty() {
        return Ok(None);
    }

    let mut tokens = trimmed.split_whitespace();
    let number_str = tokens
        .next()
        .ok_or(DurationValueParserError::NumberNotFound)?;
    let unit_str = tokens
        .next()
        .ok_or(DurationValueParserError::UnitNotFound)?;

    let number = number_str
        .parse::<i64>()
        .map_err(|source| DurationValueParserError::NumberParseFailed {
            source,
        })?;

    let duration = match unit_str.to_ascii_lowercase().as_str() {
        "second" | "seconds" => Duration::seconds(number),
        "minute" | "minutes" => Duration::minutes(number),
        "hour" | "hours" => Duration::hours(number),
        "day" | "days" => Duration::days(number),
        _ => {
            return Err(DurationValueParserError::UnitUnexpected {
                unit: unit_str.to_owned(),
            });
        }
    };

    Ok(Some(duration))
}

#[derive(Debug, Error)]
pub enum DurationValueParserError {
    #[error("duration value does not contain a number")]
    NumberNotFound,
    #[error("duration value does not contain a unit")]
    UnitNotFound,
    #[error("failed to parse duration number: {source}")]
    NumberParseFailed {
        #[source]
        source: ParseIntError,
    },
    #[error("unexpected duration unit: {unit}")]
    UnitUnexpected { unit: String },
}
