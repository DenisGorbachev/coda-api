use crate::types::ScalarValue;
use crate::{RichSingleValue, RichValue};
use thiserror::Error;

use RichSingleValue::*;
use RichValue::*;
use ScalarValue::Number;

#[derive(Debug, Error)]
pub enum ConvertRichValueRefToF64Error {
    #[error("invalid input")]
    InvalidInput,
}

#[derive(Debug, Error)]
pub enum ConvertRichValueToF64Error {
    #[error("invalid input")]
    InvalidInput { rich_value: RichValue },
}

impl TryFrom<&RichValue> for f64 {
    type Error = ConvertRichValueRefToF64Error;

    fn try_from(value: &RichValue) -> Result<Self, Self::Error> {
        use ConvertRichValueRefToF64Error::*;
        match value {
            Single(Scalar(Number(number))) => Ok(*number),
            _ => Err(InvalidInput),
        }
    }
}

impl TryFrom<RichValue> for f64 {
    type Error = ConvertRichValueToF64Error;

    fn try_from(value: RichValue) -> Result<Self, Self::Error> {
        use ConvertRichValueToF64Error::*;
        match value {
            Single(Scalar(Number(number))) => Ok(number),
            rich_value => Err(InvalidInput {
                rich_value,
            }),
        }
    }
}
