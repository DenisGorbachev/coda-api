use crate::ext::rich_rows::{RichSingleValue, RichValue};
use crate::types::ScalarValue;
use derive_more::Error;
use fmt_derive::Display;

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

impl From<&RichValue> for Result<String, ConvertRichValueToStringError> {
    fn from(value: &RichValue) -> Self {
        String::try_from(value)
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

impl From<&RichValue> for Result<Option<bool>, ConvertRichValueToOptionBoolError> {
    fn from(value: &RichValue) -> Self {
        Option::<bool>::try_from(value)
    }
}

#[cfg(feature = "time")]
mod time_impls {
    use super::*;
    use derive_more::Error;
    use error_handling::handle;
    use fmt_derive::Display;
    use std::num::ParseIntError;
    use time::format_description::well_known::Rfc3339;
    use time::{Duration, OffsetDateTime};

    impl TryFrom<&RichValue> for Option<Duration> {
        type Error = ConvertRichValueToOptionDurationError;

        fn try_from(value: &RichValue) -> Result<Self, Self::Error> {
            use ConvertRichValueToOptionDurationError::*;
            let string = handle!(String::try_from(value), ConvertStringFailed);
            let trimmed = string.trim();
            if trimmed.is_empty() {
                return Ok(None);
            }

            let mut splinters = trimmed.split_whitespace();
            let number_str = splinters.next().ok_or(NumberNotFound)?;
            let unit_str = splinters.next().ok_or(UnitNotFound)?;
            let number = handle!(number_str.parse::<i64>(), NumberParseFailed);

            let duration = match unit_str.to_ascii_lowercase().as_str() {
                "second" | "seconds" => Duration::seconds(number),
                "minute" | "minutes" => Duration::minutes(number),
                "hour" | "hours" => Duration::hours(number),
                "day" | "days" => Duration::days(number),
                _ => {
                    return Err(UnitUnexpected {
                        unit: unit_str.to_owned(),
                    });
                }
            };

            Ok(Some(duration))
        }
    }

    impl From<&RichValue> for Result<Option<Duration>, ConvertRichValueToOptionDurationError> {
        fn from(value: &RichValue) -> Self {
            Option::<Duration>::try_from(value)
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

    impl From<&RichValue> for Result<Option<OffsetDateTime>, ConvertRichValueToOptionOffsetDateTimeError> {
        fn from(value: &RichValue) -> Self {
            Option::<OffsetDateTime>::try_from(value)
        }
    }

    #[derive(Error, Display, Debug)]
    pub enum ConvertRichValueToOptionDurationError {
        ConvertStringFailed { source: ConvertRichValueToStringError },
        NumberNotFound,
        UnitNotFound,
        NumberParseFailed { source: ParseIntError },
        UnitUnexpected { unit: String },
    }

    #[derive(Error, Display, Debug)]
    pub enum ConvertRichValueToOptionOffsetDateTimeError {
        ConvertStringFailed { source: ConvertRichValueToStringError },
        OffsetDateTimeParseFailed { source: time::error::Parse, value: String },
    }
}

#[cfg(feature = "time")]
pub use time_impls::*;

#[derive(Error, Display, Debug)]
pub enum ConvertRichValueToStringError {
    RichValueCollection { rich_value: RichValue },
    RichSingleValueNotScalar { rich_single_value: RichSingleValue },
    ScalarNotString { scalar_value: ScalarValue },
}

#[derive(Error, Display, Debug)]
pub enum ConvertRichValueToOptionBoolError {
    RichValueCollection { rich_value: RichValue },
    RichSingleValueNotScalar { rich_single_value: RichSingleValue },
    ScalarNotBoolean { scalar_value: ScalarValue },
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
