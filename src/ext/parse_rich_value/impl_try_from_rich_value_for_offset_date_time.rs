use crate::{ConvertRichValueRefToStringError, ConvertRichValueToStringError, RichValue};
use errgonomic::{handle, handle_bool};
use thiserror::Error;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

#[derive(Debug, Error)]
pub enum ConvertRichValueRefToOffsetDateTimeError {
    #[error("failed to convert rich value to string")]
    ConvertStringFailed { source: ConvertRichValueRefToStringError },
    #[error("timestamp string is empty")]
    OffsetDateTimeEmptyInvalid,
    #[error("failed to parse RFC3339 timestamp")]
    OffsetDateTimeParseFailed { source: time::error::Parse },
}

#[derive(Debug, Error)]
pub enum ConvertRichValueToOffsetDateTimeError {
    #[error("failed to convert rich value to string")]
    ConvertStringFailed { source: ConvertRichValueToStringError },
    #[error("timestamp string is empty")]
    OffsetDateTimeEmptyInvalid { value: String },
    #[error("failed to parse RFC3339 timestamp '{value}'")]
    OffsetDateTimeParseFailed { source: time::error::Parse, value: String },
}

impl TryFrom<&RichValue> for OffsetDateTime {
    type Error = ConvertRichValueRefToOffsetDateTimeError;

    fn try_from(value: &RichValue) -> Result<Self, Self::Error> {
        use ConvertRichValueRefToOffsetDateTimeError::*;
        let string = handle!(String::try_from(value), ConvertStringFailed);
        let trimmed = string.trim();
        handle_bool!(trimmed.is_empty(), OffsetDateTimeEmptyInvalid);
        Ok(handle!(Self::parse(trimmed, &Rfc3339), OffsetDateTimeParseFailed))
    }
}

impl TryFrom<RichValue> for OffsetDateTime {
    type Error = ConvertRichValueToOffsetDateTimeError;

    fn try_from(value: RichValue) -> Result<Self, Self::Error> {
        use ConvertRichValueToOffsetDateTimeError::*;
        let string = handle!(String::try_from(value), ConvertStringFailed);
        let trimmed = string.trim();
        let trimmed_owned = trimmed.to_owned();
        handle_bool!(trimmed.is_empty(), OffsetDateTimeEmptyInvalid, value: trimmed_owned);
        Ok(handle!(Self::parse(trimmed, &Rfc3339), OffsetDateTimeParseFailed, value: trimmed_owned))
    }
}
