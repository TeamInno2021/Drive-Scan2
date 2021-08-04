mod fallback;
mod ffi;
mod re;

#[cfg(not(feature = "use-fallback"))]
#[cfg(windows)]
mod windows;
#[cfg(not(feature = "use-fallback"))]
#[cfg(windows)]
use windows as interface;

#[cfg(not(feature = "use-fallback"))]
#[cfg(unix)]
mod unix;
#[cfg(not(feature = "use-fallback"))]
#[cfg(unix)]
use unix as interface;

#[cfg(feature = "use-fallback")]
use fallback as interface;

#[macro_use]
extern crate napi_derive;
#[macro_use]
extern crate tracing;

use serde::Serialize;
use std::path::{Path, PathBuf};
use std::sync::atomic::{self, AtomicUsize};
use std::time::Instant;

/// Store the previous scanner used
static SCANNER: AtomicUsize = AtomicUsize::new(0);

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Scanner {
    Unknown = 0,
    Windows = 1,
    Unix = 2,
    Fallback = 3,
}

impl From<usize> for Scanner {
    fn from(n: usize) -> Self {
        match n {
            1 => Scanner::Windows,
            2 => Scanner::Unix,
            3 => Scanner::Fallback,
            _ => Scanner::Unknown,
        }
    }
}

// ------------------------------------------------------------

#[derive(Debug, Serialize)]
pub struct File {
    path: PathBuf,
    size: usize,
    directory: bool,
}

#[derive(Debug, Serialize)]
pub struct Directory {
    path: PathBuf,
    size: usize,
    files: Vec<File>,
}

// ------------------------------------------------------------

#[derive(Debug, Serialize)]
pub struct FilePtr<'f> {
    path: &'f Path,
    size: usize,
}

#[derive(Debug, Serialize)]
pub struct DirectoryPtr<'f> {
    path: &'f Path,
    size: usize,
    files: &'f [FilePtr<'f>],
}

// ------------------------------------------------------------

/// Init the logging framework and other diagnostic tools (todo)
pub fn __init() {
    tracing_subscriber::fmt::init();
    info!("Started new instance");
}

pub fn scan(dir: PathBuf) -> ::std::result::Result<(), Box<dyn ::std::error::Error>> {
    // Time the scanner
    let start = Instant::now();

    let scanner;

    match interface::verify(&dir) {
        Ok(v) => {
            if v {
                interface::scan(dir.clone())?;
                scanner = if cfg!(windows) {
                    Scanner::Windows
                } else if cfg!(unix) {
                    Scanner::Unix
                } else {
                    Scanner::Unknown
                }
            } else {
                fallback::scan(dir.clone())?;
                scanner = Scanner::Fallback;
            }
        }
        Err(err) => {
            error!("{}, using fallback...", err);
            fallback::scan(dir.clone())?;
            scanner = Scanner::Fallback;
        }
    };

    let end = start.elapsed();
    info!(
        "Scan finished in {} seconds ({} milliseconds)",
        end.as_secs(),
        end.as_millis()
    );

    SCANNER.store(scanner as usize, atomic::Ordering::SeqCst);
    Ok(())
}

pub fn query(
    dir: PathBuf,
) -> ::std::result::Result<Option<Directory>, Box<dyn ::std::error::Error>> {
    match SCANNER.load(atomic::Ordering::SeqCst).into() {
        Scanner::Unknown => {
            Err("attempted to call query(_) before calling scan(_), this is likely a bug".into())
        }
        Scanner::Fallback => fallback::query(dir),
        _ => interface::query(dir),
    }
}

// ------------------------------------------------------------

#[module_exports]
fn init(mut exports: napi::JsObject) -> napi::Result<()> {
    exports.create_named_method("init", ffi::init)?;
    exports.create_named_method("scan", ffi::scan)?;
    exports.create_named_method("query", ffi::query)?;
    Ok(())
}
