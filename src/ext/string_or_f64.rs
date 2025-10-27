use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum StringOrF64 {
    Num(f64),
    Str(String),
}

/// This function is a workaround for Coda API bug: it may return an empty string if an f64 value doesn't exist
pub(crate) fn opt_f64_from_string_or_f64<'de, D>(d: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let v: Option<StringOrF64> = Option::deserialize(d)?;
    Ok(match v {
        None => None,
        Some(StringOrF64::Num(n)) => Some(n),
        Some(StringOrF64::Str(s)) => {
            let s = s.trim();
            if s.is_empty() {
                None
            } else {
                s.parse::<f64>()
                    .map(Some)
                    .map_err(serde::de::Error::custom)?
            }
        }
    })
}
