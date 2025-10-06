# Error handling guidelines

* Don't use `?` try operator - use the macros that begin with `handle`
* Use `handle!` to unwrap `Result` types
* Use `handle_opt!` to unwrap `Option` types
* Use `handle_bool!` to return an error if some condition is true
* Use `handle_iter!` or `handle_iter_of_refs!` to collect and return errors from iterators
* Note that macros that begin with `handle` already contain a `return` statement
* Don't call `.clone()` on the variables passed into error handling macros (there is no need to clone the variables because the macros consume them only in the error branch). The macros do not consume the variables that are passed into them in the success branch. If you call a macro, you can always use the variables that are passed into the macro call in the subsequent code as if they haven't been moved (because they actually are not moved in the success branch, only in the error branch).
* Use `thiserror` to derive `Error`
* Use `thiserror` version `2.0`
* Do not annotate any error enum variant fields with a `#[from]` attribute
* Do annotate every error enum variant with an `#[error]` attribute
  * The `#[error]` attribute must contain the error message displayed for the user
  * The `#[error]` attribute must not contain the `source` field
  * The `#[error]` attribute should contain only those fields that can be displayed on one line
  * If the `#[error]` attribute contains fields, then those fields must be wrapped in single quotes. This is necessary to correctly display fields that may contain spaces.
    * Good: `#[error("user '{name}' not found")]`
    * Bad: `#[error("user {name} not found")]`

# Files

## File: src/functions/exit_result.rs
```rust
use crate::eprintln_error;
use std::error::Error;
use std::process::ExitCode;

pub fn exit_result<E: Error>(result: Result<(), E>) -> ExitCode {
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln_error(&err);
            ExitCode::FAILURE
        }
    }
}
```

## File: src/functions/get_root_error.rs
```rust
use std::error::Error;

pub fn get_root_source(error: &dyn Error) -> &dyn Error {
    let mut source = error;
    while let Some(source_new) = source.source() {
        source = source_new;
    }
    source
}
```

## File: src/functions/write_to_named_temp_file.rs
```rust
use crate::{handle, map_err};
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use tempfile::{NamedTempFile, PersistError};
use thiserror::Error;

pub fn write_to_named_temp_file(buf: &[u8]) -> Result<(File, PathBuf), WriteErrorDebugToTempFileError> {
    use WriteErrorDebugToTempFileError::*;
    let mut temp = handle!(NamedTempFile::new(), CreateTempFileFailed);
    handle!(temp.write_all(buf), WriteFailed);
    map_err!(temp.keep(), KeepFailed)
}

#[derive(Error, Debug)]
pub enum WriteErrorDebugToTempFileError {
    #[error("failed to create a temporary file")]
    CreateTempFileFailed { source: io::Error },
    #[error("failed to write to a temporary file")]
    WriteFailed { source: io::Error },
    #[error("failed to persist the temporary file")]
    KeepFailed { source: PersistError },
}
```

## File: src/types/debug_as_display.rs
```rust
use std::fmt::{Debug, Display, Formatter};

/// This wrapper is needed for types that have an easy-to-understand `Display` impl but hard-to-understand `Debug` impl
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct DebugAsDisplay<T: Display>(pub T);

impl<T: Display> Debug for DebugAsDisplay<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl<T: Display> Display for DebugAsDisplay<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl<T: Display> From<T> for DebugAsDisplay<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}
```

## File: src/types/display_as_debug.rs
```rust
use std::fmt::{Debug, Display, Formatter};

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
pub struct DisplayAsDebug<T: Debug>(pub T);

impl<T: Debug> Display for DisplayAsDebug<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl<T: Debug> From<T> for DisplayAsDebug<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}
```

## File: src/types/display_debug_pair.rs
```rust
use std::fmt::{Debug, Display};

#[derive(Clone, Debug)]
pub struct DisplayDebugPair<T: Display + Debug> {
    pub display: String,
    pub debug: T,
}

impl<T: Display + Debug> From<T> for DisplayDebugPair<T> {
    fn from(value: T) -> Self {
        Self {
            display: value.to_string(),
            debug: value,
        }
    }
}
```

## File: src/functions.rs
```rust
mod get_root_error;

pub use get_root_error::*;

mod eprintln_error;

pub use eprintln_error::*;

mod write_to_named_temp_file;

pub use write_to_named_temp_file::*;

mod exit_result;

pub use exit_result::*;
```

## File: src/functions/eprintln_error.rs
```rust
use crate::functions::write_to_named_temp_file;
use std::error::Error;

pub fn eprintln_error(error: &dyn Error) {
    eprintln!("- {}", error);
    let mut source = error;
    while let Some(source_new) = source.source() {
        eprintln!("- {}", source_new);
        source = source_new;
    }
    eprintln!();
    let error_debug = format!("{:#?}", error);
    let result = write_to_named_temp_file::write_to_named_temp_file(error_debug.as_bytes());
    match result {
        Ok((_file, path_buf)) => {
            eprintln!("See the full error report:\nless {}", path_buf.display());
        }
        Err(other_error) => {
            eprintln!("{other_error:#?}");
        }
    }
}
```

## File: src/types/item_error.rs
```rust
use thiserror::Error;

#[derive(Error, Debug)]
#[error("error occurred for item {item}: {source}")]
pub struct ItemError<T, E> {
    pub item: T,
    pub source: E,
}
```

## File: src/types.rs
```rust
mod debug_as_display;
mod display_as_debug;
mod display_debug_pair;
mod item_error;

pub use debug_as_display::*;
pub use display_as_debug::*;
pub use display_debug_pair::*;
pub use item_error::*;

use std::path::PathBuf;

pub type PathBufDisplay = DisplayAsDebug<PathBuf>;
```

## File: src/macros.rs
```rust
/// [`handle!`](crate::handle) is a better alternative to [`map_err`](Result::map_err) because it doesn't capture any variables from the environment if the result is [`Ok`], only when the result is [`Err`].
/// By contrast, a closure passed to `map_err` always captures the variables from environment, regardless of whether the result is [`Ok`] or [`Err`]
/// Use [`handle!`](crate::handle) if you need to pass owned variables to an error variant (which is returned only in case when result is [`Err`])
/// In addition, this macro captures the original error in the `source` variable, and sets it as the `source` key of the error variant
///
/// Note: [`handle!`](crate::handle) assumes that your error variant is a struct variant
#[macro_export]
macro_rules! handle {
    ($result:expr, $variant:ident$(,)? $($arg:ident$(: $value:expr)?),*) => {
        match $result {
            Ok(value) => value,
            Err(source) => return Err($variant {
                source: source.into(),
                $($arg: $crate::_into!($arg$(: $value)?)),*
            }),
        }
    };
}

#[macro_export]
macro_rules! handle_opt {
    ($option:expr, $variant:ident$(,)? $($arg:ident$(: $value:expr)?),*) => {
        match $option {
            Some(value) => value,
            None => return Err($variant {
                $($arg: $crate::_into!($arg$(: $value)?)),*
            }),
        }
    };
}

#[macro_export]
macro_rules! handle_bool {
    ($condition:expr, $variant:ident$(,)? $($arg:ident$(: $value:expr)?),*) => {
        if $condition {
            return Err($variant {
                $($arg: $crate::_into!($arg$(: $value)?)),*
            });
        };
    };
}

/// `$results` must be an `impl Iterator<Item = Result<T, E>>`
#[macro_export]
macro_rules! handle_iter {
    ($results:expr, $variant:ident$(,)? $($arg:ident$(: $value:expr)?),*) => {
        {
            let (oks, errors): (Vec<_>, Vec<_>) = itertools::Itertools::partition_result($results);
            if errors.is_empty() {
                oks
            } else {
                return Err($variant {
                    sources: errors.into(),
                    $($arg: $crate::_into!($arg$(: $value)?)),*
                });
            }
        }
    };
}

/// Note that this macro returns an expression that evaluates to a tuple of `(outputs, items)`. This is necessary because the iteration consumes items, which might actually be relevant to the subsequent code
/// If the errors are empty, then `items.len() == outputs.len()`
/// Note that the `results` iterator might abort early without consuming all items. In this case, the `items` will contain less elements than prior to this macro invocation
#[macro_export]
macro_rules! handle_iter_of_refs {
    ($results:expr, $items:expr, $variant:ident $(, $arg:ident$(: $value:expr)?)*) => {
        {
            let mut outputs = Vec::new();
            let mut items = Vec::new();
            let mut errors = Vec::new();
            for (result, item) in std::iter::zip($results, $items) {
                match result {
                    Ok(output) => {
                        outputs.push(output);
                        items.push(item);
                    },
                    Err(source) => {
                        errors.push($crate::ItemError {
                            item,
                            source,
                        });
                    }
                }
            }
            if errors.is_empty() {
                (outputs, items)
            } else {
                return Err($variant {
                    sources: errors.into(),
                    $($arg: $crate::_into!($arg$(: $value)?)),*
                });
            }
        }
    };
}

/// `$results` must be an `impl IntoIterator<Item = Result<T, E>>`
#[macro_export]
macro_rules! handle_into_iter {
    ($results:expr, $variant:ident $(, $arg:ident$(: $value:expr)?)*) => {
        $crate::handle_iter!($results.into_iter(), $variant $(, $arg$(: $value)?),*)
    };
}

/// [`handle_discard`](crate::handle_discard) should only be used when you want to discard the source error. This is discouraged. Prefer other handle-family macros that preserve the source error.
#[macro_export]
macro_rules! handle_discard {
    ($result:expr, $variant:ident$(,)? $($arg:ident$(: $value:expr)?),*) => {
        match $result {
            Ok(value) => value,
            Err(_) => return Err($variant {
                $($arg: $crate::_into!($arg$(: $value)?)),*
            }),
        }
    };
}

/// [`map_err`](crate::map_err) should be used only when the error variant doesn't capture any owned variables (which is very rare), or exactly at the end of the block (in the position of returned expression).
#[macro_export]
macro_rules! map_err {
    ($result:expr, $variant:ident$(,)? $($arg:ident$(: $value:expr)?),*) => {
        $result.map_err(|source| $variant {
            source: source.into(),
            $($arg: $crate::into!($arg$(: $value)?)),*
        })
    };
}

/// Internal
#[macro_export]
macro_rules! _into {
    ($arg:ident) => {
        $arg.into()
    };
    ($arg:ident: $value:expr) => {
        $value.into()
    };
}

/// Internal
#[macro_export]
macro_rules! _index_err {
    ($f:ident) => {
        |(index, item)| $f(item).map_err(|err| (index, err))
    };
}

/// Internal
#[macro_export]
macro_rules! _index_err_async {
    ($f:ident) => {
        async |(index, item)| $f(item).await.map_err(|err| (index, err))
    };
}

#[cfg(test)]
mod tests {
    use crate::ItemError;
    use futures::future::join_all;
    use serde::{Deserialize, Serialize};
    use std::io;
    use std::path::{Path, PathBuf};
    use std::str::FromStr;
    use std::sync::{Arc, RwLock};
    use thiserror::Error;
    use tokio::fs::read_to_string;
    use tokio::task::JoinSet;

    #[allow(dead_code)]
    struct PrintNameCommand {
        dir: PathBuf,
        format: Format,
    }

    #[allow(dead_code)]
    impl PrintNameCommand {
        async fn run(self) -> Result<(), PrintNameCommandError> {
            use PrintNameCommandError::*;
            let Self {
                dir,
                format,
            } = self;
            let config = handle!(parse_config(&dir, format).await, ParseConfigFailed);
            println!("{}", config.name);
            Ok(())
        }
    }

    /// This function tests the [`crate::handle!`] macro
    #[allow(dead_code)]
    async fn parse_config(dir: &Path, format: Format) -> Result<Config, ParseConfigError> {
        use Format::*;
        use ParseConfigError::*;
        let path_buf = dir.join("config.json");
        let contents = handle!(read_to_string(&path_buf).await, ReadFileFailed, path: path_buf);
        match format {
            Json => {
                let config = handle!(serde_json::de::from_str(&contents), DeserializeFromJson, path: path_buf, contents);
                Ok(config)
            }
            Toml => {
                let config = handle!(toml::de::from_str(&contents), DeserializeFromToml, path: path_buf, contents);
                Ok(config)
            }
        }
    }

    /// This function tests the [`crate::handle_opt!`] macro
    #[allow(dead_code)]
    fn find_even(numbers: Vec<u32>) -> Result<u32, FindEvenError> {
        use FindEvenError::*;
        let even = handle_opt!(numbers.iter().find(|x| *x % 2 == 0), NotFound);
        Ok(*even)
    }

    /// This function tests the [`crate::handle_iter!`] macro
    #[allow(dead_code)]
    fn multiply_evens(numbers: Vec<u32>) -> Result<Vec<u32>, MultiplyEvensError> {
        use MultiplyEvensError::*;
        let results = numbers.into_iter().map(|number| {
            use CheckEvenError::*;
            if number % 2 == 0 {
                Ok(number * 10)
            } else {
                Err(NumberNotEven {
                    number,
                })
            }
        });
        Ok(handle_iter!(results, CheckEvensFailed))
    }

    /// This function tests the [`crate::handle_into_iter!`] macro
    #[allow(dead_code)]
    async fn read_files(paths: Vec<PathBuf>) -> Result<Vec<String>, ReadFilesError> {
        use ReadFilesError::*;
        let results = paths
            .into_iter()
            .map(check_file)
            .collect::<JoinSet<_>>()
            .join_all()
            .await;
        Ok(handle_into_iter!(results, CheckFileFailed))
    }

    #[allow(dead_code)]
    async fn read_files_ref(paths: Vec<PathBuf>) -> Result<Vec<String>, ReadFilesRefError> {
        use ReadFilesRefError::*;
        let iter = paths.iter().map(check_file_ref);
        let results = join_all(iter).await;
        let (outputs, _paths) = handle_iter_of_refs!(results.into_iter(), paths, CheckFileRefFailed);
        Ok(outputs)
    }

    // async fn check_file(path: &Path)

    /// This function exists to test error handling in async code
    #[allow(dead_code)]
    async fn process(number: u32) -> Result<u32, ProcessError> {
        Ok(number)
    }

    #[derive(Error, Debug)]
    enum PrintNameCommandError {
        #[error("failed to parse config")]
        ParseConfigFailed { source: ParseConfigError },
    }

    /// Variants don't have the `format` field because every variant already corresponds to a single specific format
    /// Some variants have the `path` field because the `contents` depends on `path`
    /// `path` has type `PathBufDisplay` because `PathBuf` doesn't implement `Display`
    /// Some `source` field types are wrapped in `Box` according to suggestion from `result_large_err` lint
    #[derive(Error, Debug)]
    enum ParseConfigError {
        #[error("failed to read the file: {path}")]
        ReadFileFailed { path: PathBuf, source: io::Error },
        #[error("failed to deserialize the file contents from JSON: {path}")]
        DeserializeFromJson { path: PathBuf, contents: String, source: Box<serde_json::error::Error> },
        #[error("failed to deserialize the file contents from TOML: {path}")]
        DeserializeFromToml { path: PathBuf, contents: String, source: Box<toml::de::Error> },
    }

    #[allow(dead_code)]
    #[derive(Error, Debug)]
    enum ProcessError {}

    #[allow(dead_code)]
    #[derive(Copy, Clone, Debug)]
    enum Format {
        Json,
        Toml,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    struct Config {
        name: String,
        timeout: u64,
        parallel: bool,
    }

    #[allow(dead_code)]
    fn parse_even_number(input: &str) -> Result<u32, ParseEvenNumberError> {
        use ParseEvenNumberError::*;
        let number = handle!(input.parse::<u32>(), InputParseFailed);
        handle_bool!(number % 2 != 0, NumberNotEven, number);
        Ok(number)
    }

    #[derive(Error, Debug)]
    enum ParseEvenNumberError {
        #[error("failed to parse input")]
        InputParseFailed { source: <u32 as FromStr>::Err },
        #[error("number is not even: {number}")]
        NumberNotEven { number: u32 },
    }

    #[derive(Error, Debug)]
    enum FindEvenError {
        #[error("even number not found")]
        NotFound,
    }

    #[derive(Error, Debug)]
    enum MultiplyEvensError {
        #[error("failed to check {len} numbers", len = .sources.len())]
        CheckEvensFailed { sources: Vec<CheckEvenError> },
    }

    #[derive(Error, Debug)]
    enum ReadFilesError {
        #[error("failed to check {len} files", len = .sources.len())]
        CheckFileFailed { sources: Vec<CheckFileError> },
    }

    #[derive(Error, Debug)]
    enum ReadFilesRefError {
        #[error("failed to check {len} files", len = .sources.len())]
        CheckFileRefFailed { sources: Vec<ItemError<PathBuf, CheckFileRefError>> },
    }

    #[derive(Error, Debug)]
    enum CheckEvenError {
        #[error("number is not even: {number}")]
        NumberNotEven { number: u32 },
    }

    async fn check_file(path: PathBuf) -> Result<String, CheckFileError> {
        use CheckFileError::*;
        let content = handle!(read_to_string(&path).await, ReadToStringFailed, path);
        handle_bool!(content.is_empty(), FileIsEmpty, path);
        Ok(content)
    }

    #[derive(Error, Debug)]
    enum CheckFileError {
        #[error("failed to read the file to string: {path}")]
        ReadToStringFailed { path: PathBuf, source: io::Error },
        #[error("file is empty: {path}")]
        FileIsEmpty { path: PathBuf },
    }

    async fn check_file_ref(path: &PathBuf) -> Result<String, CheckFileRefError> {
        use CheckFileRefError::*;
        let content = handle!(read_to_string(&path).await, ReadToStringFailed);
        handle_bool!(content.is_empty(), FileIsEmpty);
        Ok(content)
    }

    #[derive(Error, Debug)]
    enum CheckFileRefError {
        #[error("failed to read the file to string")]
        ReadToStringFailed { source: io::Error },
        #[error("file is empty")]
        FileIsEmpty,
    }

    #[derive(Clone, Debug)]
    struct Db {
        user: User,
    }

    #[derive(Clone, Debug)]
    struct User {
        username: String,
    }

    #[allow(dead_code)]
    fn get_username(db: Arc<RwLock<Db>>) -> Result<String, GetUsernameError> {
        use GetUsernameError::*;
        // `db.read()` returns `LockResult` whose Err variant is `PoisonError<RwLockReadGuard<'_, T>>`, which contains an anonymous lifetime
        // The error enum returned from this function must contain only owned fields, so it can't contain a `source` that has a lifetime
        // Therefore, we have to use handle_discard!, although it is discouraged
        let guard = handle_discard!(db.read(), AcquireReadLockFailed);
        let username = guard.user.username.clone();
        Ok(username)
    }

    #[derive(Error, Debug)]
    pub enum GetUsernameError {
        #[error("failed to acquire read lock")]
        AcquireReadLockFailed,
    }
}
```

## File: src/lib.rs
```rust
//! # Error handling
//!
//! ## Goal
//!
//! Help the caller diagnose the issue, fix it, and retry the call.
//!
//! ## Approach
//!
//! Every error must be represented by a unique enum variant with relevant fields.
//!
//! ## Guidelines
//!
//! ### General
//!
//! * Every error type must be an enum
//! * Every error enum variant must be a struct variant
//! * Every error enum variant must contain one field per owned variable that is relevant to the fallible expression that this variant wraps
//!   * The relevant variable is a variable whose value determines whether the fallible expression returns an [`Ok`] or an [`Err`]
//! * Every error enum variant must have fields only for [`data types`](#data-type), not for [`non-data types`](#non-data-type)
//! * Every error enum variant field must have an owned type (not a reference)
//! * Every error enum should be located below the function that returns it (in the same file)
//! * Every fallible function must return a unique error type
//! * Every call to another fallible function must be wrapped in a unique error enum variant
//! * If the function contains only one fallible expression, this expression must still be wrapped in an error enum variant
//! * Every variable that contains secret data (the one which must not be displayed or logged, e.g. password, API key, personally identifying information) must have a type that doesn't output the underlying data in the Debug and Display impls (e.g. [`secrecy::SecretBox`](https://docs.rs/secrecy/latest/secrecy/struct.SecretBox.html))
//! * The code that calls a fallible function on each element of a collection should return an `impl Iterator<Item = Result<T, E>>` instead of short-circuiting on the first error
//! * If Clippy outputs a `result_large_err` warning, then the large fields of the error enum must be wrapped in a `Box`
//! * If the error enum variant has a `source` field, then this field must be the first field
//! * The code must not use strings for error messages
//! * The production code must not use `unwrap` or `expect` (only tests may use `unwrap` or `expect`)
//! * If each field of each variant of the error enum implements `Copy`, then the error enum must implement `Copy` too
//! * If an argument of callee implements `Copy`, the callee must not include it in the list of error enum variant fields (the caller must include it because of the rule to include all relevant owned variables)
//!
//! ### Conveniences
//!
//! * Every fallible function body must begin with `use ThisFunctionError::*;`, where `ThisFunctionError` must be the name of this function's error enum (for example: `use ParseConfigError::*;`)
//! * The error handling code must use the error enum variant names without the error enum name prefix (for example: `ReadFileFailed` instead of `ParseConfigError::ReadFileFailed`)
//!
//! ### Naming
//!
//! * The name of the error enum must end with `Error` (for example: `ParseConfigError`)
//! * The name of the error enum variant should end with `Failed` or `NotFound` or `Invalid` (for example: `ReadFileFailed`, `UserNotFound`, `PasswordInvalid`)
//! * If the error variant name is associated with a child function call, the name of the error variant must be equal to the name of the function converted to CamelCase concatenated with `Failed` (for example: if the parent function calls `read_file`, then it should call it like this: `handle!(read_file(&path), ReadFileFailed, path)`
//! * The name of the error enum must include the name of the function converted to CamelCase
//!   * If the function is a freestanding function, the name of the error type must be exactly equal to the name of the function converted to CamelCase concatenated with `Error`
//!   * If the function is an associated function, the name of the error type must be exactly equal to the name of the type without generics concatenated with the name of the function in CamelCase concatenated with `Error`
//!   * If the error is specified as an associated type of a foreign trait with multiple functions that return this associated error type, then the name of the error type must be exactly equal to the name of the trait including generics concatenated with the name of the type for which this trait is implemented concatenated with `Error`
//! * If the error enum is defined for a `TryFrom<A> for B` impl, then its name must be equal to "Convert{A}To{B}Error"
//!
//! ## Macros
//!
//! Use the following macros for more concise error handling:
//!
//! * [`handle!`] instead of [`Result::map_err`]
//! * [`handle_opt!`] instead of [`Option::ok_or`] and [`Option::ok_or_else`]
//! * [`handle_bool!`] instead of `if condition { return Err(...) }`
//! * [`handle_iter!`] instead of code that handles errors in iterators
//! * [`handle_iter_of_refs!`] instead of the code handles errors in iterators of references (where the values are still being owned by the underlying collection)
//! * [`handle_into_iter!`] replaces the code that handles errors in collections that implement [`IntoIterator`] (including [`Vec`] and [`HashMap`](std::collections::HashMap)
//!
//! ## Definitions
//!
//! ### Fallible expression
//!
//! An expression that returns a [`Result`].
//!
//! ### Data type
//!
//! A type that holds the actual data.
//!
//! For example:
//!
//! * `bool`
//! * `String`
//! * `PathBuf`
//!
//! ### Non-data type
//!
//! A type that doesn't hold the actual data.
//!
//! For example:
//!
//! * `RestClient` doesn't point to the actual data, it only allows querying it.
//! * `DatabaseConnection` doesn't hold the actual data, it only allows querying it.

mod macros;

mod types;

pub use types::*;

mod functions;

pub use functions::*;
```
