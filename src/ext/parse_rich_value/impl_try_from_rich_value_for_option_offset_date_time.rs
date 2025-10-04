use crate::{ConvertRichValueRefToStringError, ConvertRichValueToStringError, RichValue};
use error_handling::handle;
use thiserror::Error;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

#[derive(Debug, Error)]
pub enum ConvertRichValueRefToOptionOffsetDateTimeError {
    #[error("failed to convert rich value to string")]
    ConvertStringFailed { source: ConvertRichValueRefToStringError },
    #[error("failed to parse RFC3339 timestamp")]
    OffsetDateTimeParseFailed { source: time::error::Parse },
}

#[derive(Debug, Error)]
pub enum ConvertRichValueToOptionOffsetDateTimeError {
    #[error("failed to convert rich value to string")]
    ConvertStringFailed { source: ConvertRichValueToStringError },
    #[error("failed to parse RFC3339 timestamp '{value}'")]
    OffsetDateTimeParseFailed { source: time::error::Parse, value: String },
}

impl TryFrom<&RichValue> for Option<OffsetDateTime> {
    type Error = ConvertRichValueRefToOptionOffsetDateTimeError;

    fn try_from(value: &RichValue) -> Result<Self, Self::Error> {
        use ConvertRichValueRefToOptionOffsetDateTimeError::*;
        let string = handle!(String::try_from(value), ConvertStringFailed);
        let trimmed = string.trim();
        if trimmed.is_empty() {
            return Ok(None);
        }

        let value = handle!(OffsetDateTime::parse(trimmed, &Rfc3339), OffsetDateTimeParseFailed);
        Ok(Some(value))
    }
}

impl TryFrom<RichValue> for Option<OffsetDateTime> {
    type Error = ConvertRichValueToOptionOffsetDateTimeError;

    fn try_from(value: RichValue) -> Result<Self, Self::Error> {
        use ConvertRichValueToOptionOffsetDateTimeError::*;
        let string = handle!(String::try_from(value), ConvertStringFailed);
        let trimmed = string.trim();
        if trimmed.is_empty() {
            return Ok(None);
        }

        let trimmed_owned = trimmed.to_owned();
        let parsed = handle!(OffsetDateTime::parse(trimmed, &Rfc3339), OffsetDateTimeParseFailed, value: trimmed_owned);
        Ok(Some(parsed))
    }
}
