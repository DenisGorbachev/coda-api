use governor::{DefaultDirectRateLimiter, Quota};
use std::num::NonZeroU32;
use std::time::Duration;

// TODO: add other rate limiters based on Coda API docs
#[derive(Debug)]
pub struct Limiter {
    pub read: DefaultDirectRateLimiter,
    pub write: DefaultDirectRateLimiter,
}

impl Default for Limiter {
    fn default() -> Self {
        let write_quota = Quota::with_period(Duration::new(3, 0)).expect("Duration is not empty");
        let write = DefaultDirectRateLimiter::direct(write_quota);
        let read_duration = Duration::new(6, 0);
        let read_burst = NonZeroU32::new(100).expect("not zero");
        let read_quota = Quota::with_period(read_duration)
            .expect("not empty")
            .allow_burst(read_burst);
        let read = DefaultDirectRateLimiter::direct(read_quota);
        Self {
            read,
            write,
        }
    }
}
