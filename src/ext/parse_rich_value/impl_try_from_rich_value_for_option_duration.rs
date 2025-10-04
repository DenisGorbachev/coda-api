use crate::{ConvertRichValueRefToStringError, ConvertRichValueToStringError, DurationValueParserError, RichValue, parse_duration_value};
use error_handling::handle;
use thiserror::Error;
use time::Duration;

#[derive(Debug, Error)]
pub enum ConvertRichValueRefToOptionDurationError {
    #[error("failed to convert rich value to string: {source}")]
    ConvertStringFailed {
        #[source]
        source: ConvertRichValueRefToStringError,
    },
    #[error("failed to parse duration value: {source}")]
    DurationParseFailed {
        #[source]
        source: DurationValueParserError,
    },
}

#[derive(Debug, Error)]
pub enum ConvertRichValueToOptionDurationError {
    #[error("failed to convert rich value to string: {source}")]
    ConvertStringFailed {
        #[source]
        source: ConvertRichValueToStringError,
    },
    #[error("failed to parse duration value '{value}': {source}")]
    DurationParseFailed {
        #[source]
        source: DurationValueParserError,
        value: String,
    },
}

impl TryFrom<&RichValue> for Option<Duration> {
    type Error = ConvertRichValueRefToOptionDurationError;

    fn try_from(value: &RichValue) -> Result<Self, Self::Error> {
        use ConvertRichValueRefToOptionDurationError::{ConvertStringFailed, DurationParseFailed};

        let string = handle!(String::try_from(value), ConvertStringFailed);
        Ok(handle!(parse_duration_value(&string), DurationParseFailed))
    }
}

impl TryFrom<RichValue> for Option<Duration> {
    type Error = ConvertRichValueToOptionDurationError;

    fn try_from(value: RichValue) -> Result<Self, Self::Error> {
        use ConvertRichValueToOptionDurationError::{ConvertStringFailed, DurationParseFailed};

        let string = handle!(String::try_from(value), ConvertStringFailed);
        Ok(handle!(parse_duration_value(&string), DurationParseFailed, value: string))
    }
}
