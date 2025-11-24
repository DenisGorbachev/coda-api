use governor::{DefaultDirectRateLimiter, Quota};
use std::num::NonZeroU32;
use std::time::Duration;

#[derive(Debug)]
pub struct Limiter {
    pub read: DefaultDirectRateLimiter,
    pub write: DefaultDirectRateLimiter,
    pub doc_content_write: DefaultDirectRateLimiter,
    pub list_docs: DefaultDirectRateLimiter,
    pub analytics: DefaultDirectRateLimiter,
}

impl Default for Limiter {
    fn default() -> Self {
        let read = make_burst_limiter(100, Duration::from_secs(6));
        let write = make_burst_limiter(10, Duration::from_secs(6));
        let doc_content_write = make_burst_limiter(5, Duration::from_secs(10));
        let list_docs = make_burst_limiter(4, Duration::from_secs(6));
        let analytics = make_burst_limiter(100, Duration::from_secs(6));
        Self {
            read,
            write,
            doc_content_write,
            list_docs,
            analytics,
        }
    }
}

fn make_burst_limiter(burst: u32, period: Duration) -> DefaultDirectRateLimiter {
    let burst = NonZeroU32::new(burst).expect("burst must be non-zero");
    let quota = Quota::with_period(period)
        .expect("period must be non-zero")
        .allow_burst(burst);
    DefaultDirectRateLimiter::direct(quota)
}
