use crate::types::ScalarValue;
use crate::{RichSingleValue, RichValue};
use thiserror::Error;

use RichSingleValue::*;
use RichValue::*;
use ScalarValue::*;

#[derive(Debug, Error)]
pub enum ConvertRichValueRefToStringError {
    #[error("rich value is a collection")]
    RichValueCollection,
    #[error("rich single value is not scalar")]
    RichSingleValueNotScalar,
    #[error("scalar value is not a string")]
    ScalarNotString,
}

#[derive(Debug, Error)]
pub enum ConvertRichValueToStringError {
    #[error("rich value is a collection")]
    RichValueCollection { rich_value: RichValue },
    #[error("rich single value is not scalar")]
    RichSingleValueNotScalar { rich_single_value: RichSingleValue },
    #[error("scalar value is not a string")]
    ScalarNotString { scalar_value: ScalarValue },
}

impl TryFrom<&RichValue> for String {
    type Error = ConvertRichValueRefToStringError;

    fn try_from(value: &RichValue) -> Result<Self, Self::Error> {
        use ConvertRichValueRefToStringError::*;
        match value {
            Single(single) => match single {
                Scalar(scalar) => match scalar {
                    Variant0(text) => Ok(super::normalize_rich_string(text.as_str())),
                    _ => Err(ScalarNotString),
                },
                _ => Err(RichSingleValueNotScalar),
            },
            _ => Err(RichValueCollection),
        }
    }
}

impl TryFrom<RichValue> for String {
    type Error = ConvertRichValueToStringError;

    fn try_from(value: RichValue) -> Result<Self, Self::Error> {
        use ConvertRichValueToStringError::*;
        match value {
            Single(single) => match single {
                Scalar(scalar) => match scalar {
                    Variant0(text) => Ok(super::normalize_owned_rich_string(text)),
                    scalar_value => Err(ScalarNotString {
                        scalar_value,
                    }),
                },
                rich_single_value => Err(RichSingleValueNotScalar {
                    rich_single_value,
                }),
            },
            rich_value => Err(RichValueCollection {
                rich_value,
            }),
        }
    }
}
