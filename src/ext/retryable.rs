use crate::types::{GetMutationStatusResponse, GetRowResponse, ListRowsResponse};

pub trait Retryable {
    fn should_retry(&self) -> bool;
}

macro_rules! impl_retryable {
    ($($response:ty),* $(,)?) => {
        $(impl $crate::Retryable for $crate::Error<$response> {
            fn should_retry(&self) -> bool {
                self.is_retryable()
                    || matches!(self, $crate::Error::ErrorResponse(response)
                        if response.status() == reqwest::StatusCode::BAD_REQUEST
                            && response.message == "Doc is not yet up to date.")
            }
        })*
    };
}

impl_retryable!(GetMutationStatusResponse, GetRowResponse, ListRowsResponse);
