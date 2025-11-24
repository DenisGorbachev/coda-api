# Implement a rate-limiting `Client` for Coda API

## Files

* src/client.rs
* src/limiter.rs
* src/ext.rs
* src/lib.rs

## Tasks

* Add other rate limiters based on Coda API docs in `Limiter`
* Ensure that `Client` has every method that is present on the `RawClient` struct (impl blocks both in src/lib.rs and in src/ext.rs), but before calling `RawClient` (before proxying to the `raw` field) it must call the appropriate rate limiter and must `await` the rate limiter.
* In the file `src/ext.rs` there is some code with methods which must also be present on the `Client` struct. Some of these methods are pagination methods and they must call the methods on `Client` itself, not on `RawClient`, so that rate limits are enforced. Move them into `Client`
* Methods in `src/ext.rs` that have the `correct` postfix must be kept as they are and must not be changed.
