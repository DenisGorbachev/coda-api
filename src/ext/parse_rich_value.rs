mod impl_try_from_rich_value_for_option_bool;
pub use impl_try_from_rich_value_for_option_bool::*;

#[cfg(feature = "time")]
mod impl_try_from_rich_value_for_option_duration;
#[cfg(feature = "time")]
pub use impl_try_from_rich_value_for_option_duration::*;

#[cfg(feature = "time")]
mod impl_try_from_rich_value_for_option_offset_date_time;
#[cfg(feature = "time")]
pub use impl_try_from_rich_value_for_option_offset_date_time::*;

mod impl_try_from_rich_value_for_rich_row_reference;
pub use impl_try_from_rich_value_for_rich_row_reference::*;

mod impl_try_from_rich_value_for_string;
pub use impl_try_from_rich_value_for_string::*;

pub fn normalize_rich_string(value: &str) -> String {
    if let Some(stripped) = value
        .strip_prefix("```")
        .and_then(|inner| inner.strip_suffix("```"))
    {
        stripped.to_string()
    } else {
        value.to_string()
    }
}

pub fn normalize_owned_rich_string(value: String) -> String {
    if value.len() >= 6 && value.starts_with("```") && value.ends_with("```") {
        value[3..value.len() - 3].to_string()
    } else {
        value
    }
}
