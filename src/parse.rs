use crate::types::{CellValue, RichSingleValue, RichValue, ScalarValue, Value, ValueVariant0};
use derive_more::Error;
use fmt_derive::Display;

pub enum CodaType {
    String,
    Duration,
}

impl TryFrom<CellValue> for String {
    type Error = ConvertCellValueToStringError;

    fn try_from(cell_value: CellValue) -> Result<Self, Self::Error> {
        use ConvertCellValueToStringError::*;
        match cell_value {
            CellValue::Value(Value::Variant0(ValueVariant0::Variant0(string))) => Ok(string),
            CellValue::RichValue(RichValue::Variant0(RichSingleValue::ScalarValue(ScalarValue::Variant0(string)))) => Ok(string),
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
    use time::OffsetDateTime;

    impl TryFrom<CellValue> for Option<OffsetDateTime> {
        type Error = ConvertCellValueToOffsetDateTime;

        fn try_from(value: CellValue) -> Result<Self, Self::Error> {
            use ConvertCellValueToOffsetDateTime::*;
            let string = handle!(String::try_from(value), ConvertCellValueToStringFailed);
            if string.is_empty() {
                Ok(None)
            } else {
                let value = handle!(OffsetDateTime::parse(&string, &Rfc3339), OffsetDateTimeParseFailed);
                Ok(Some(value))
            }
        }
    }

    #[derive(Error, Display, Debug)]
    pub enum ConvertCellValueToOffsetDateTime {
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
