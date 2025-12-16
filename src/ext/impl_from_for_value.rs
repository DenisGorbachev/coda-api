use crate::types::{Value, ValueVariant0};

impl From<String> for ValueVariant0 {
    fn from(value: String) -> Self {
        Self::Variant0(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::Variant0(value.into())
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Variant0(value.into())
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Variant0(value.into())
    }
}
