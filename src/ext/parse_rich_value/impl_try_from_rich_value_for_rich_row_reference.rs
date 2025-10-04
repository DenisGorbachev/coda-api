use crate::{RichRowReference, RichSingleValue, RichValue};
use thiserror::Error;

use RichSingleValue::*;
use RichValue::*;

#[derive(Debug, Error)]
pub enum ConvertRichValueRefToRichRowReferenceError {
    #[error("rich value is not a rich row reference")]
    RichValueIsNotRichRowReference,
}

#[derive(Debug, Error)]
pub enum ConvertRichValueToRichRowReferenceError {
    #[error("rich value is not a rich row reference")]
    RichValueIsNotRichRowReference { rich_value: RichValue },
}

impl TryFrom<&RichValue> for RichRowReference {
    type Error = ConvertRichValueRefToRichRowReferenceError;

    fn try_from(value: &RichValue) -> Result<Self, Self::Error> {
        use ConvertRichValueRefToRichRowReferenceError::*;
        match value {
            Single(Row(reference)) => Ok(reference.clone()),
            _ => Err(RichValueIsNotRichRowReference),
        }
    }
}

impl TryFrom<RichValue> for RichRowReference {
    type Error = ConvertRichValueToRichRowReferenceError;

    fn try_from(value: RichValue) -> Result<Self, Self::Error> {
        use ConvertRichValueToRichRowReferenceError::*;
        match value {
            Single(Row(reference)) => Ok(reference),
            rich_value => Err(RichValueIsNotRichRowReference {
                rich_value,
            }),
        }
    }
}
