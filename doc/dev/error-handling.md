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
  * If the `#[error]` attribute contains fields that implement `Display`, then those fields must be output using `Display` formatting (not `Debug` formatting)
    * Good:
      ```
      #[error("task not found for query '{query}'")]
      TaskNotFound { query: String }
      ```
    * Bad:
      ```
      #[error("task not found for query '{query:?}'")]
      TaskNotFound { query: String }
      ```
  * If the `#[error]` attribute contains fields, then those fields must be wrapped in single quotes. This is necessary to correctly display fields that may contain spaces.
    * Good: `#[error("user '{name}' not found")]`
    * Bad: `#[error("user {name} not found")]`

# Files

## File: src/drafts/err_vec_display.rs
```rust
use std::error::Error;
use std::fmt::{Display, Formatter};
use thiserror::Error;

/// This type is an attempt to implement error display for multiple errors through [`Display`] trait
#[derive(Error, Debug)]
pub struct ErrVecDisplay<T> {
    inner: Vec<T>,
}

impl<T: Error + 'static> Display for ErrVecDisplay<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("encountered {len} errors\n", len = self.inner.len()))?;
        for err in &self.inner {
            print_error(err, "  * ".to_string());
        }
        Ok(())
    }
}

pub fn print_error(error: &(dyn Error + 'static), prefix: String) {
    println!("{prefix}{error}");
    if let Some(source) = error.source() {
        print_error(source, prefix);
    }
}

#[derive(Error, Debug)]
pub enum FooError {
    #[error("bar failed")]
    BarFailed { source: BarError },
}

#[derive(Error, Debug)]
pub enum BarError {
    #[error("zeds failed")]
    ZedsFailed { source: ErrVecDisplay<ZedError> },
}

#[derive(Error, Debug)]
pub enum ZedError {
    #[error("ksa failed")]
    KsaFailed,
    #[error("pry failed")]
    PryFailed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn must_err_vec_try() {
        let error = FooError::BarFailed {
            source: BarError::ZedsFailed {
                source: ErrVecDisplay {
                    inner: vec![ZedError::KsaFailed, ZedError::PryFailed],
                },
            },
        };
        print_error(&error, "- ".to_string());
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

## File: src/types/path_buf_display.rs
```rust
use crate::DisplayAsDebug;
use std::path::PathBuf;

pub type PathBufDisplay = DisplayAsDebug<PathBuf>;
```

## File: src/drafts.rs
```rust
pub mod err_vec_display;
```

## File: src/functions/writeln_error/must_write_error.txt
```
- failed to run CLI command
- failed to run i18n update command
- failed to update 2 rows
  * - failed to send an i18n request for row 'Foo'
    - failed to construct a JSON schema
    - input must be an object
  * - failed to send an i18n request for row 'Bar'
    - failed to send a request
    - address 239.143.73.1 is not available
```

## File: src/functions/exit_result.rs
```rust
use crate::eprintln_error;
use std::error::Error;
use std::process::ExitCode;

pub fn exit_result<E: Error + 'static>(result: Result<(), E>) -> ExitCode {
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln_error(&err);
            ExitCode::FAILURE
        }
    }
}
```

## File: src/functions/writeln_error.rs
```rust
use crate::functions::write_to_named_temp_file;
use crate::{ErrVec, Prefixer};
use std::error::Error;
use std::io;
use std::io::{Write, stderr};

pub fn writeln_error_to_writer_and_file(error: &(dyn Error + 'static), writer: &mut dyn Write) -> Result<(), io::Error> {
    writeln_error_to_writer(error, writer, true)?;
    writeln!(writer)?;
    let error_debug = format!("{error:#?}");
    let result = write_to_named_temp_file(error_debug.as_bytes());
    match result {
        Ok((_file, path_buf)) => {
            writeln!(writer, "See the full error report:\nless {}", path_buf.display())
        }
        Err(other_error) => {
            writeln!(writer, "{other_error:#?}")
        }
    }
}

pub fn writeln_error_to_writer(error: &(dyn Error + 'static), writer: &mut dyn Write, is_top_level: bool) -> Result<(), io::Error> {
    let source = error;
    if let Some(err_vec) = source.downcast_ref::<ErrVec>() {
        if is_top_level {
            writeln!(writer, "- {error}")?;
        }
        for err in &err_vec.inner {
            let mut prefixer = error_prefixer(writer);
            writeln_error_to_writer(err.as_ref(), &mut prefixer, false)?;
        }
        Ok(())
    } else {
        writeln!(writer, "- {error}")?;
        if let Some(source_new) = source.source() {
            writeln_error_to_writer(source_new, writer, false)
        } else {
            Ok(())
        }
    }
}

pub fn eprintln_error(error: &(dyn Error + 'static)) {
    let mut stderr = stderr().lock();
    let result = writeln_error_to_writer_and_file(error, &mut stderr);
    match result {
        Ok(()) => (),
        Err(err) => eprintln!("failed to write to stderr: {err:#?}"),
    }
}

pub fn error_prefixer(writer: &mut dyn Write) -> Prefixer<'_> {
    Prefixer::new("  * ", "    ", writer)
}

#[cfg(test)]
mod tests {
    use crate::functions::writeln_error::tests::JsonSchemaNewError::InputMustBeObject;
    use crate::{ErrVec, writeln_error_to_writer};
    use thiserror::Error;

    #[test]
    fn must_write_error() {
        let error = CliRunError::CommandRunFailed {
            source: CommandRunError::I18nUpdateRunFailed {
                source: I18nUpdateRunError::UpdateRowsFailed {
                    source: vec![
                        UpdateRowError::I18nRequestFailed {
                            source: I18nRequestError::JsonSchemaNewFailed {
                                source: InputMustBeObject {
                                    input: "foo".to_string(),
                                },
                            },
                            row: Row::new("Foo"),
                        },
                        UpdateRowError::I18nRequestFailed {
                            source: I18nRequestError::RequestSendFailed {
                                source: tokio::io::Error::new(tokio::io::ErrorKind::AddrNotAvailable, "address 239.143.73.1 is not available"),
                            },
                            row: Row::new("Bar"),
                        },
                    ]
                    .into(),
                },
            },
        };
        let mut output = Vec::new();
        writeln_error_to_writer(&error, &mut output, true).unwrap();
        let string = String::from_utf8(output).unwrap();
        assert_eq!(string, include_str!("writeln_error/must_write_error.txt"))
    }

    #[derive(Error, Debug)]
    pub enum CliRunError {
        #[error("failed to run CLI command")]
        CommandRunFailed { source: CommandRunError },
    }

    #[derive(Error, Debug)]
    pub enum CommandRunError {
        #[error("failed to run i18n update command")]
        I18nUpdateRunFailed { source: I18nUpdateRunError },
    }

    #[derive(Error, Debug)]
    pub enum I18nUpdateRunError {
        #[error("failed to update {len} rows", len = source.len())]
        UpdateRowsFailed { source: ErrVec },
    }

    #[derive(Error, Debug)]
    pub enum UpdateRowError {
        #[error("failed to send an i18n request for row '{row}'", row = row.name)]
        I18nRequestFailed { source: I18nRequestError, row: Row },
    }

    #[derive(Error, Debug)]
    pub enum I18nRequestError {
        #[error("failed to construct a JSON schema")]
        JsonSchemaNewFailed { source: JsonSchemaNewError },
        #[error("failed to send a request")]
        RequestSendFailed { source: tokio::io::Error },
    }

    #[derive(Error, Debug)]
    pub enum JsonSchemaNewError {
        #[error("input must be an object")]
        InputMustBeObject { input: String },
    }

    #[derive(Debug)]
    pub struct Row {
        name: String,
    }

    impl Row {
        pub fn new(name: impl Into<String>) -> Self {
            Self {
                name: name.into(),
            }
        }
    }
}
```

## File: src/types/err_vec.rs
```rust
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

#[derive(Default, Debug)]
pub struct ErrVec {
    pub inner: Vec<Box<dyn Error + 'static>>,
}

impl Display for ErrVec {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("encountered {len} errors", len = self.inner.len()))
    }
}

impl Error for ErrVec {}

impl Deref for ErrVec {
    type Target = Vec<Box<dyn Error + 'static>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for ErrVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl ErrVec {
    pub fn new<E: Error + 'static>(iter: impl IntoIterator<Item = E>) -> Self {
        Self {
            inner: iter
                .into_iter()
                .map(|err| Box::new(err) as Box<dyn Error + 'static>)
                .collect(),
        }
    }
}

impl<E: Error + 'static> From<Vec<E>> for ErrVec {
    fn from(value: Vec<E>) -> Self {
        Self::new(value)
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

## File: src/types/prefixer.rs
```rust
use std::fmt;
use std::io::{self, Write};

/// This type uses a `dyn Write` instead of `impl Write` to avoid a trait-recursion explosion in [`crate::writeln_error_to_writer`]
pub struct Prefixer<'w> {
    pub first_line_prefix: String,
    pub next_line_prefix: String,
    pub writer: &'w mut dyn Write,
    pub is_first_line: bool,
    pub needs_prefix: bool,
}

impl<'w> fmt::Debug for Prefixer<'w> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Prefixer")
            .field("first_line_prefix", &self.first_line_prefix)
            .field("next_line_prefix", &self.next_line_prefix)
            .field("is_first_line", &self.is_first_line)
            .field("needs_prefix", &self.needs_prefix)
            .finish()
    }
}

impl<'w> Prefixer<'w> {
    pub fn new(first_line_prefix: impl Into<String>, next_line_prefix: impl Into<String>, writer: &'w mut dyn Write) -> Self {
        Self {
            first_line_prefix: first_line_prefix.into(),
            next_line_prefix: next_line_prefix.into(),
            writer,
            is_first_line: true,
            needs_prefix: true,
        }
    }
}

impl<'w> Write for Prefixer<'w> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }

        let mut start = 0;
        while start < buf.len() {
            if self.needs_prefix {
                let prefix = if self.is_first_line { &self.first_line_prefix } else { &self.next_line_prefix };
                self.writer.write_all(prefix.as_bytes())?;
                self.is_first_line = false;
                self.needs_prefix = false;
            }

            match buf[start..].iter().position(|&b| b == b'\n') {
                Some(relative_idx) => {
                    let end = start + relative_idx + 1;
                    self.writer.write_all(&buf[start..end])?;
                    start = end;
                    self.needs_prefix = true;
                }
                None => {
                    self.writer.write_all(&buf[start..])?;
                    start = buf.len();
                }
            }
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}
```

## File: src/functions.rs
```rust
mod get_root_error;

pub use get_root_error::*;

mod writeln_error;

pub use writeln_error::*;

mod write_to_named_temp_file;

pub use write_to_named_temp_file::*;

mod exit_result;

pub use exit_result::*;
```

## File: src/types.rs
```rust
mod debug_as_display;
mod display_as_debug;
mod display_debug_pair;
mod err_vec;
mod item_error;
mod path_buf_display;
mod prefixer;

pub use debug_as_display::*;
pub use display_as_debug::*;
pub use display_debug_pair::*;
pub use err_vec::*;
pub use item_error::*;
pub use path_buf_display::*;
pub use prefixer::*;
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

/// See also: [`handle_opt_take!`](crate::handle_opt_take)
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

/// This macro is an opposite of [`handle_opt!`](crate::handle_opt) - it returns an error if the option contains a `Some` variant.
///
/// Note that this macro calls [`Option::take`], which will leave a `None` if the option was `Some(value)`.
/// Note that this macro has a mandatory argument `$some_value` (used in `if let Some($some_value) = $option.take()`), which will also be passed to the error enum variant.
#[macro_export]
macro_rules! handle_opt_take {
    ($option:expr, $variant:ident, $some_value:ident$(,)? $($arg:ident$(: $value:expr)?),*) => {
        if let Some($some_value) = $option.take() {
            return Err($variant {
                $some_value: $some_value.into(),
                $($arg: $crate::_into!($arg$(: $value)?)),*
            })
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
                    source: errors.into(),
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
                    source: errors.into(),
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
            $($arg: $crate::_into!($arg$(: $value)?)),*
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
    use crate::{ErrVec, PathBufDisplay};
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
        let items = paths.into_iter().map(PathBufDisplay::from);
        let (outputs, _items) = handle_iter_of_refs!(results.into_iter(), items, CheckFileRefFailed);
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
        #[error("failed to check {len} numbers", len = source.len())]
        CheckEvensFailed { source: ErrVec },
    }

    #[derive(Error, Debug)]
    enum ReadFilesError {
        #[error("failed to check {len} files", len = source.len())]
        CheckFileFailed { source: ErrVec },
    }

    #[derive(Error, Debug)]
    enum ReadFilesRefError {
        #[error("failed to check {len} files", len = source.len())]
        CheckFileRefFailed { source: ErrVec },
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

    #[allow(dead_code)]
    fn get_answer(prompt: String, get_response: &mut impl FnMut(String) -> Result<WeirdResponse, io::Error>) -> Result<String, GetAnswerError> {
        use GetAnswerError::*;
        // Since the `get_response` external API doesn't return the `prompt` in its error, we have to clone `prompt` before passing it as argument, so that we could pass it to the error enum variant
        // Cloning may be necessary with external APIs that don't return arguments in errors, but it must not be necessary in our code
        let mut response = handle!(get_response(prompt.clone()), GetResponseFailed, prompt);
        handle_opt_take!(response.error, ResponseContainsError, error);
        Ok(response.answer)
    }

    /// OpenAI Responses API returns a response with `error: Option<WeirdResponseError>` field, which is weird, but must still be handled
    #[derive(Debug)]
    pub struct WeirdResponse {
        answer: String,
        error: Option<WeirdResponseError>,
    }

    #[allow(dead_code)]
    #[derive(Error, Debug)]
    pub enum WeirdResponseError {
        #[error("prompt is empty")]
        PromptIsEmpty,
        #[error("context limit reached")]
        ContextLimitReached,
    }

    /// [`GetAnswerError::GetResponseFailed`] `error` attribute doesn't contain a reference to `{prompt}` because the prompt can be very long, so it would make the error message very long, which is undesirable
    #[derive(Error, Debug)]
    pub enum GetAnswerError {
        #[error("failed to get response")]
        GetResponseFailed { source: io::Error, prompt: String },
        #[error("response contains an error")]
        ResponseContainsError { error: WeirdResponseError },
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

extern crate core;

mod macros;

mod types;

pub use types::*;

mod functions;

pub use functions::*;

#[cfg(test)]
mod drafts;
```
