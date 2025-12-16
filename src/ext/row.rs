use crate::ValueFormatProvider;
use crate::types::{Row, ValueFormat};

impl ValueFormatProvider for Row {
    fn value_format() -> ValueFormat {
        ValueFormat::Simple
    }
}
