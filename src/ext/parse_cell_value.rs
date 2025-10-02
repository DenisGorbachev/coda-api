use crate::types::{CellValue, RichSingleValue, RichValue, ScalarValue, Value, ValueVariant0};
use thiserror::Error;

impl TryFrom<CellValue> for String {
    type Error = ConvertCellValueToStringError;

    fn try_from(cell_value: CellValue) -> Result<Self, Self::Error> {
        use ConvertCellValueToStringError::*;
        match cell_value {
            CellValue::Value(Value::Variant0(ValueVariant0::Variant0(value))) => Ok(value),
            CellValue::RichValue(RichValue::Variant0(RichSingleValue::ScalarValue(ScalarValue::Variant0(value)))) => Ok(value),
            cell_value => Err(InvalidCellValue {
                cell_value,
            }),
        }
    }
}

impl TryFrom<CellValue> for f64 {
    type Error = ConvertCellValueToF64Error;

    fn try_from(cell_value: CellValue) -> Result<Self, Self::Error> {
        use ConvertCellValueToF64Error::*;
        match cell_value {
            CellValue::Value(Value::Variant0(ValueVariant0::Variant1(value))) => Ok(value),
            CellValue::RichValue(RichValue::Variant0(RichSingleValue::ScalarValue(ScalarValue::Variant1(value)))) => Ok(value),
            cell_value => Err(InvalidCellValue {
                cell_value,
            }),
        }
    }
}

/// Coda allows to "unset" the boolean value by hitting Delete while the cell is focused (the checkbox becomes greyed out). For ValueFormat != Rich, such "unset" values are returned as empty strings. During parsing, such values will be converted to `None`.
impl TryFrom<CellValue> for Option<bool> {
    type Error = ConvertCellValueToBoolError;

    fn try_from(cell_value: CellValue) -> Result<Self, Self::Error> {
        use ConvertCellValueToBoolError::*;
        match cell_value {
            CellValue::Value(Value::Variant0(ValueVariant0::Variant0(value))) if value.is_empty() => Ok(None),
            CellValue::Value(Value::Variant0(ValueVariant0::Variant2(value))) => Ok(Some(value)),
            CellValue::RichValue(RichValue::Variant0(RichSingleValue::ScalarValue(ScalarValue::Variant2(value)))) => Ok(Some(value)),
            cell_value => Err(InvalidCellValue {
                cell_value,
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

    impl TryFrom<CellValue> for Option<OffsetDateTime> {
        type Error = ConvertCellValueToOptionOffsetDateTimeError;

        fn try_from(value: CellValue) -> Result<Self, Self::Error> {
            use ConvertCellValueToOptionOffsetDateTimeError::*;
            let string = handle!(String::try_from(value), ConvertCellValueToStringFailed);
            if string.is_empty() {
                Ok(None)
            } else {
                let value = handle!(OffsetDateTime::parse(&string, &Rfc3339), OffsetDateTimeParseFailed);
                Ok(Some(value))
            }
        }
    }

    impl TryFrom<CellValue> for Option<Duration> {
        type Error = ConvertCellValueToOptionDurationError;

        fn try_from(value: CellValue) -> Result<Self, Self::Error> {
            use ConvertCellValueToOptionDurationError::{ConvertCellValueToStringFailed, DurationParseFailed};

            let string = handle!(String::try_from(value), ConvertCellValueToStringFailed);
            Ok(handle!(parse_duration_value(&string), DurationParseFailed))
        }
    }

    #[derive(Debug, Error)]
    pub enum ConvertCellValueToOptionDurationError {
        #[error("failed to convert cell value to duration string: {source}")]
        ConvertCellValueToStringFailed {
            #[source]
            source: ConvertCellValueToStringError,
        },
        #[error("failed to parse duration value: {source}")]
        DurationParseFailed {
            #[source]
            source: DurationValueParserError,
        },
    }

    #[derive(Debug, Error)]
    pub enum ConvertCellValueToOptionOffsetDateTimeError {
        #[error("failed to convert cell value to timestamp string: {source}")]
        ConvertCellValueToStringFailed {
            #[source]
            source: ConvertCellValueToStringError,
        },
        #[error("failed to parse RFC3339 timestamp: {source}")]
        OffsetDateTimeParseFailed {
            #[source]
            source: time::Error,
        },
    }
}

#[cfg(feature = "time")]
pub use time_impls::*;

#[derive(Debug, Error)]
pub enum ConvertCellValueToStringError {
    #[error("cell value is not a string: {cell_value:?}")]
    InvalidCellValue { cell_value: CellValue },
}

#[derive(Debug, Error)]
pub enum ConvertCellValueToF64Error {
    #[error("cell value is not a number: {cell_value:?}")]
    InvalidCellValue { cell_value: CellValue },
}

#[derive(Debug, Error)]
pub enum ConvertCellValueToBoolError {
    #[error("cell value is not a boolean: {cell_value:?}")]
    InvalidCellValue { cell_value: CellValue },
}

#[derive(Debug, Error)]
pub enum ConvertCellValueError {
    #[error("failed to convert cell value to string: {source}")]
    ConvertCellValueToStringFailed {
        #[source]
        source: ConvertCellValueToStringError,
    },
    #[error("failed to convert cell value to number: {source}")]
    ConvertCellValueToF64Failed {
        #[source]
        source: ConvertCellValueToF64Error,
    },
    #[error("failed to convert cell value to boolean: {source}")]
    ConvertCellValueToBoolFailed {
        #[source]
        source: ConvertCellValueToBoolError,
    },
    #[cfg(feature = "time")]
    #[error("failed to convert cell value to timestamp: {source}")]
    ConvertCellValueToOffsetDateTimeFailed {
        #[source]
        source: ConvertCellValueToOptionOffsetDateTimeError,
    },
}
