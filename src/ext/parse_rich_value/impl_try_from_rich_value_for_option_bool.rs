use crate::types::ScalarValue;
use crate::{RichSingleValue, RichValue};
use thiserror::Error;

use RichSingleValue::*;
use RichValue::*;
use ScalarValue::*;

#[derive(Debug, Error)]
pub enum ConvertRichValueRefToOptionBoolError {
    #[error("rich value is a collection")]
    RichValueCollection,
    #[error("rich single value is not scalar")]
    RichSingleValueNotScalar,
    #[error("scalar value is not a boolean")]
    ScalarNotBoolean,
    #[error("string scalar is not empty")]
    StringScalarNotEmpty,
}

#[derive(Debug, Error)]
pub enum ConvertRichValueToOptionBoolError {
    #[error("rich value is a collection: {rich_value:?}")]
    RichValueCollection { rich_value: RichValue },
    #[error("rich single value is not scalar: {rich_single_value:?}")]
    RichSingleValueNotScalar { rich_single_value: RichSingleValue },
    #[error("scalar value is not a boolean: {scalar_value:?}")]
    ScalarNotBoolean { scalar_value: ScalarValue },
    #[error("string scalar is not empty: {text}")]
    StringScalarNotEmpty { text: String },
}

impl TryFrom<&RichValue> for Option<bool> {
    type Error = ConvertRichValueRefToOptionBoolError;

    fn try_from(value: &RichValue) -> Result<Self, Self::Error> {
        use ConvertRichValueRefToOptionBoolError::*;
        match value {
            Single(single) => match single {
                Scalar(scalar) => match scalar {
                    Variant2(boolean) => Ok(Some(*boolean)),
                    Variant0(text) if text.trim().is_empty() => Ok(None),
                    Variant0(_) => Err(StringScalarNotEmpty),
                    _ => Err(ScalarNotBoolean),
                },
                _ => Err(RichSingleValueNotScalar),
            },
            _ => Err(RichValueCollection),
        }
    }
}

impl TryFrom<RichValue> for Option<bool> {
    type Error = ConvertRichValueToOptionBoolError;

    fn try_from(value: RichValue) -> Result<Self, Self::Error> {
        use ConvertRichValueToOptionBoolError::*;
        match value {
            Single(single) => match single {
                Scalar(scalar) => match scalar {
                    Variant2(boolean) => Ok(Some(boolean)),
                    Variant0(text) => {
                        if text.trim().is_empty() {
                            Ok(None)
                        } else {
                            Err(StringScalarNotEmpty {
                                text,
                            })
                        }
                    }
                    scalar_value => Err(ScalarNotBoolean {
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
