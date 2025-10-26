use crate::types::ScalarValue;
use crate::{RichSingleValue, RichValue};
use thiserror::Error;

use RichSingleValue::*;
use RichValue::*;
use ScalarValue::*;

#[derive(Debug, Error)]
pub enum ConvertRichValueRefToBoolError {
    #[error("invalid input")]
    InvalidInput,
}

#[derive(Debug, Error)]
pub enum ConvertRichValueToBoolError {
    #[error("invalid input")]
    InvalidInput { rich_value: RichValue },
}

impl TryFrom<&RichValue> for bool {
    type Error = ConvertRichValueRefToBoolError;

    fn try_from(value: &RichValue) -> Result<Self, Self::Error> {
        use ConvertRichValueRefToBoolError::*;
        match value {
            Single(Scalar(Variant2(boolean))) => Ok(*boolean),
            _ => Err(InvalidInput),
        }
    }
}

impl TryFrom<RichValue> for bool {
    type Error = ConvertRichValueToBoolError;

    fn try_from(value: RichValue) -> Result<Self, Self::Error> {
        use ConvertRichValueToBoolError::*;
        match value {
            Single(Scalar(Variant2(boolean))) => Ok(boolean),
            rich_value => Err(InvalidInput {
                rich_value,
            }),
        }
    }
}
