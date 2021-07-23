#[cfg(windows)]
mod windows;
#[cfg(windows)]
use windows as interface;

#[cfg(unix)]
mod unix;
#[cfg(unix)]
use unix as interface;

mod fallback;

#[macro_use]
extern crate napi_derive;
#[macro_use]
extern crate tracing;

use napi::{CallContext, JsObject, JsString, JsUnknown, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub enum File {
    File {
        path: PathBuf,
        size: usize,
    },
    Directory {
        files: HashMap<String, File>,
        path: PathBuf,
        size: usize,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanResult {
    base: PathBuf,
    files: File,
}

pub fn _scan(dir: PathBuf) -> ::std::result::Result<ScanResult, Box<dyn ::std::error::Error>> {
    let res = match interface::verify(&dir) {
        Ok(v) => {
            if v {
                interface::scan(dir.clone())
            } else {
                info!("unable to verify scan directory, using fallback");
                fallback::scan(dir.clone())
            }
        }
        Err(e) => {
            error!(
                "unexpected error while attempting to validate scan directory ({}): {}",
                std::env::consts::OS,
                e
            );
            fallback::scan(dir.clone())
        }
    };

    match res {
        Ok(f) => Ok(ScanResult {
            base: dir,
            files: f,
        }),
        Err(e) => Err(e),
    }
}

#[js_function(1)]
fn scan(ctx: CallContext) -> Result<JsUnknown> {
    let dir: PathBuf = ctx.get::<JsString>(0)?.into_utf8()?.as_str()?.into();

    match _scan(dir) {
        Ok(f) => ctx.env.to_js_value(&f),
        Err(e) => Err(napi::Error::new(napi::Status::Unknown, e.to_string())),
    }
}

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
    tracing_subscriber::fmt::init();
    exports.create_named_method("scan", scan)?;
    Ok(())
}
