use crate::types::{GetMutationStatusResponse, GetRowResponse, ListRowsResponse};

macro_rules! impl_is_retryable {
    ($($response:ty),* $(,)?) => {
        $(impl retry_please::IsRetryable for $response {
            fn is_retryable(&self) -> bool {
                self.status_code == 400.0 && self.message == "Doc is not yet up to date."
            }
        })*
    };
}

impl_is_retryable!(GetMutationStatusResponse, GetRowResponse, ListRowsResponse);
