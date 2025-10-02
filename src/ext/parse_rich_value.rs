use crate::ext::rich_rows::{RichSingleValue, RichValue};
use crate::types::ScalarValue;
use thiserror::Error;

use RichSingleValue::*;
use RichValue::*;
use ScalarValue::*;

impl TryFrom<&RichValue> for String {
    type Error = ConvertRichValueToStringError;

    fn try_from(value: &RichValue) -> Result<Self, Self::Error> {
        use ConvertRichValueToStringError::*;
        match value {
            Single(single) => match single {
                Scalar(scalar) => match scalar {
                    Variant0(text) => Ok(normalize_rich_string(text)),
                    _ => Err(ScalarNotString {
                        scalar_value: scalar.clone(),
                    }),
                },
                _ => Err(RichSingleValueNotScalar {
                    rich_single_value: single.clone(),
                }),
            },
            _ => Err(RichValueCollection {
                rich_value: value.clone(),
            }),
        }
    }
}

impl TryFrom<&RichValue> for Option<bool> {
    type Error = ConvertRichValueToOptionBoolError;

    fn try_from(value: &RichValue) -> Result<Self, Self::Error> {
        use ConvertRichValueToOptionBoolError::*;
        match value {
            Single(single) => match single {
                Scalar(scalar) => match scalar {
                    Variant2(boolean) => Ok(Some(*boolean)),
                    Variant0(text) if text.trim().is_empty() => Ok(None),
                    Variant0(_) => Err(StringScalarNotEmpty {
                        scalar_value: scalar.clone(),
                    }),
                    _ => Err(ScalarNotBoolean {
                        scalar_value: scalar.clone(),
                    }),
                },
                _ => Err(RichSingleValueNotScalar {
                    rich_single_value: single.clone(),
                }),
            },
            _ => Err(RichValueCollection {
                rich_value: value.clone(),
            }),
        }
    }
}

#[cfg(feature = "time")]
mod time_impls {
    use super::*;
    use crate::{DurationValueParserError, parse_duration_value};
    use error_handling::handle;
    use thiserror::Error;
    use time::format_description::well_known::Rfc3339;
    use time::{Duration, OffsetDateTime};

    impl TryFrom<&RichValue> for Option<Duration> {
        type Error = ConvertRichValueToOptionDurationError;

        fn try_from(value: &RichValue) -> Result<Self, Self::Error> {
            use ConvertRichValueToOptionDurationError::{ConvertStringFailed, DurationParseFailed};

            let string = handle!(String::try_from(value), ConvertStringFailed);
            Ok(handle!(parse_duration_value(&string), DurationParseFailed))
        }
    }

    impl TryFrom<&RichValue> for Option<OffsetDateTime> {
        type Error = ConvertRichValueToOptionOffsetDateTimeError;

        fn try_from(value: &RichValue) -> Result<Self, Self::Error> {
            use ConvertRichValueToOptionOffsetDateTimeError::*;
            let string = handle!(String::try_from(value), ConvertStringFailed);
            let trimmed = string.trim();
            if trimmed.is_empty() {
                return Ok(None);
            }

            let value = handle!(OffsetDateTime::parse(trimmed, &Rfc3339), OffsetDateTimeParseFailed, value: trimmed.to_owned());
            Ok(Some(value))
        }
    }

    #[derive(Debug, Error)]
    pub enum ConvertRichValueToOptionDurationError {
        #[error("failed to convert rich value to string: {source}")]
        ConvertStringFailed {
            #[source]
            source: ConvertRichValueToStringError,
        },
        #[error("failed to parse duration value: {source}")]
        DurationParseFailed {
            #[source]
            source: DurationValueParserError,
        },
    }

    #[derive(Debug, Error)]
    pub enum ConvertRichValueToOptionOffsetDateTimeError {
        #[error("failed to convert rich value to string: {source}")]
        ConvertStringFailed {
            #[source]
            source: ConvertRichValueToStringError,
        },
        #[error("failed to parse RFC3339 timestamp '{value}': {source}")]
        OffsetDateTimeParseFailed {
            #[source]
            source: time::error::Parse,
            value: String,
        },
    }
}

#[cfg(feature = "time")]
pub use time_impls::*;

#[derive(Debug, Error)]
pub enum ConvertRichValueToStringError {
    #[error("rich value is a collection: {rich_value:?}")]
    RichValueCollection { rich_value: RichValue },
    #[error("rich single value is not scalar: {rich_single_value:?}")]
    RichSingleValueNotScalar { rich_single_value: RichSingleValue },
    #[error("scalar value is not a string: {scalar_value:?}")]
    ScalarNotString { scalar_value: ScalarValue },
}

#[derive(Debug, Error)]
pub enum ConvertRichValueToOptionBoolError {
    #[error("rich value is a collection: {rich_value:?}")]
    RichValueCollection { rich_value: RichValue },
    #[error("rich single value is not scalar: {rich_single_value:?}")]
    RichSingleValueNotScalar { rich_single_value: RichSingleValue },
    #[error("scalar value is not a boolean: {scalar_value:?}")]
    ScalarNotBoolean { scalar_value: ScalarValue },
    #[error("string scalar is not empty: {scalar_value:?}")]
    StringScalarNotEmpty { scalar_value: ScalarValue },
}

pub fn normalize_rich_string(value: &str) -> String {
    if let Some(stripped) = value
        .strip_prefix("```")
        .and_then(|inner| inner.strip_suffix("```"))
    {
        stripped.to_string()
    } else {
        value.to_string()
    }
}
