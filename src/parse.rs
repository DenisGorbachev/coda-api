use crate::types::{CellValue, RichSingleValue, RichValue, ScalarValue, Value, ValueVariant0};
use derive_more::Error;
use fmt_derive::Display;

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

impl TryFrom<CellValue> for bool {
    type Error = ConvertCellValueToBoolError;

    fn try_from(cell_value: CellValue) -> Result<Self, Self::Error> {
        use ConvertCellValueToBoolError::*;
        match cell_value {
            CellValue::Value(Value::Variant0(ValueVariant0::Variant2(value))) => Ok(value),
            CellValue::RichValue(RichValue::Variant0(RichSingleValue::ScalarValue(ScalarValue::Variant2(value)))) => Ok(value),
            cell_value => Err(InvalidCellValue {
                cell_value,
            }),
        }
    }
}

#[cfg(feature = "time")]
mod time_impls {
    use super::*;
    use crate::types::CellValue;
    use derive_more::Error;
    use error_handling::handle;
    use fmt_derive::Display;
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
            use ConvertCellValueToOptionDurationError::*;
            let string = handle!(String::try_from(value), ConvertCellValueToStringFailed);
            if string.is_empty() {
                Ok(None)
            } else {
                let mut splinters = string.split(' ');
                let number_str = splinters.next().ok_or(NumberNotFound)?;
                let unit_str = splinters.next().ok_or(UnitNotFound)?;
                let number = handle!(number_str.parse::<i64>(), NumberParseFailed);
                match unit_str {
                    "second" | "seconds" => Ok(Some(Duration::seconds(number))),
                    "minute" | "minutes" => Ok(Some(Duration::minutes(number))),
                    "hour" | "hours" => Ok(Some(Duration::hours(number))),
                    "day" | "days" => Ok(Some(Duration::days(number))),
                    _ => Err(UnitUnexpected),
                }
            }
        }
    }

    #[derive(Error, Display, Debug)]
    pub enum ConvertCellValueToOptionDurationError {
        ConvertCellValueToStringFailed { source: ConvertCellValueToStringError },
        NumberNotFound,
        NumberParseFailed { source: std::num::ParseIntError },
        UnitNotFound,
        UnitUnexpected,
    }

    #[derive(Error, Display, Debug)]
    pub enum ConvertCellValueToOptionOffsetDateTimeError {
        ConvertCellValueToStringFailed { source: ConvertCellValueToStringError },
        OffsetDateTimeParseFailed { source: time::Error },
    }
}

#[cfg(feature = "time")]
pub use time_impls::*;

#[derive(Error, Display, Debug)]
pub enum ConvertCellValueToStringError {
    InvalidCellValue { cell_value: CellValue },
}

#[derive(Error, Display, Debug)]
pub enum ConvertCellValueToF64Error {
    InvalidCellValue { cell_value: CellValue },
}

#[derive(Error, Display, Debug)]
pub enum ConvertCellValueToBoolError {
    InvalidCellValue { cell_value: CellValue },
}

#[derive(Error, Display, Debug)]
pub enum ConvertCellValueError {
    ConvertCellValueToStringFailed {
        source: ConvertCellValueToStringError,
    },
    ConvertCellValueToF64Failed {
        source: ConvertCellValueToF64Error,
    },
    ConvertCellValueToBoolFailed {
        source: ConvertCellValueToBoolError,
    },
    #[cfg(feature = "time")]
    ConvertCellValueToOffsetDateTimeFailed {
        source: ConvertCellValueToOptionOffsetDateTimeError,
    },
}
