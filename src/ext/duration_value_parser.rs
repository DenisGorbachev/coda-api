use errgonomic::{handle, handle_opt};
use std::num::ParseIntError;
use thiserror::Error;
use time::Duration;

const SECONDS_PER_MINUTE: i64 = 60;
const SECONDS_PER_HOUR: i64 = 3_600;
const SECONDS_PER_DAY: i64 = 86_400;

pub fn parse_duration_value(source: &impl AsRef<str>) -> Result<Option<Duration>, DurationValueParserError> {
    use DurationValueParserError::*;
    let trimmed = source.as_ref().trim();
    if trimmed.is_empty() {
        return Ok(None);
    }

    let mut tokens = trimmed.split_whitespace();
    let number_str = handle_opt!(tokens.next(), NumberNotFound);
    let unit_str = handle_opt!(tokens.next(), UnitNotFound);
    parse_duration_component(number_str, unit_str)
        .and_then(|duration_initial| {
            core::iter::from_fn(|| tokens.next().map(|number_str| (number_str, tokens.next()))).try_fold(duration_initial, |duration_total, (number_str, unit_str_opt)| {
                let unit_str = handle_opt!(unit_str_opt, UnitNotFound);
                parse_duration_component(number_str, unit_str).and_then(|duration| duration_total.checked_add(duration).ok_or(DurationOverflow))
            })
        })
        .map(Some)
}

fn parse_duration_component(number_str: &str, unit_str: &str) -> Result<Duration, DurationValueParserError> {
    use DurationValueParserError::*;
    let number = handle!(number_str.parse::<i64>(), NumberParseFailed);
    duration_from_number_and_unit(number, unit_str)
}

fn duration_from_number_and_unit(number: i64, unit_str: &str) -> Result<Duration, DurationValueParserError> {
    use DurationValueParserError::*;
    match unit_str.to_ascii_lowercase().as_str() {
        "second" | "seconds" | "sec" | "secs" => Ok(Duration::seconds(number)),
        "minute" | "minutes" | "min" | "mins" => checked_duration_from_seconds(number, SECONDS_PER_MINUTE),
        "hour" | "hours" | "hr" | "hrs" => checked_duration_from_seconds(number, SECONDS_PER_HOUR),
        "day" | "days" => checked_duration_from_seconds(number, SECONDS_PER_DAY),
        _ => Err(UnitUnexpected {
            unit: unit_str.to_owned(),
        }),
    }
}

fn checked_duration_from_seconds(number: i64, seconds_per_unit: i64) -> Result<Duration, DurationValueParserError> {
    use DurationValueParserError::*;
    match number.checked_mul(seconds_per_unit) {
        Some(seconds) => Ok(Duration::seconds(seconds)),
        None => Err(DurationOverflow),
    }
}

#[derive(Debug, PartialEq, Error)]
pub enum DurationValueParserError {
    #[error("duration value does not contain a number")]
    NumberNotFound,
    #[error("duration value does not contain a unit")]
    UnitNotFound,
    #[error("failed to parse duration number: {source}")]
    NumberParseFailed { source: ParseIntError },
    #[error("unexpected duration unit: {unit}")]
    UnitUnexpected { unit: String },
    #[error("duration value is too large")]
    DurationOverflow,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn must_parse_simple_duration() {
        assert_eq!(parse_duration_value(&"2 hrs"), Ok(Some(Duration::hours(2))));
    }

    #[test]
    fn must_parse_complex_duration() {
        assert_eq!(parse_duration_value(&"2 hrs 30 mins"), Ok(Some(Duration::hours(2) + Duration::minutes(30))));
    }
}
