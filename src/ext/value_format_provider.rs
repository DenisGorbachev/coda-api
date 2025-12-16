use crate::types::ValueFormat;

pub trait ValueFormatProvider {
    fn value_format() -> ValueFormat;
}
