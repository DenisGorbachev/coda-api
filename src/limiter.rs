use governor::{DefaultDirectRateLimiter, Quota};
use std::num::NonZeroU32;
use std::time::Duration;

/// The rate limits are slightly lower compared to official docs because I've seen "429 Too Many Requests" errors even with these rate limits
#[derive(Debug)]
pub struct Limiter {
    pub read: DefaultDirectRateLimiter,
    pub write: DefaultDirectRateLimiter,
    pub write_doc_content: DefaultDirectRateLimiter,
    pub list_docs: DefaultDirectRateLimiter,
    pub read_analytics: DefaultDirectRateLimiter,
}

/// Buffer added to official rate limits
const BUFFER: u64 = 1;

impl Default for Limiter {
    fn default() -> Self {
        let read = make_burst_limiter(100, Duration::from_secs(6 + BUFFER));
        let write = make_burst_limiter(10, Duration::from_secs(6 + BUFFER));
        let write_doc_content = make_burst_limiter(5, Duration::from_secs(10 + BUFFER));
        let list_docs = make_burst_limiter(4, Duration::from_secs(6 + BUFFER));
        let read_analytics = make_burst_limiter(100, Duration::from_secs(6 + BUFFER));
        Self {
            read,
            write,
            write_doc_content,
            list_docs,
            read_analytics,
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
