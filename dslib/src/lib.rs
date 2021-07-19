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

use napi::{CallContext, JsObject, JsString, JsUnknown, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub enum File {
    File { path: PathBuf, size: usize },
    Directory(HashMap<String, File>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanResult {
    base: PathBuf,
    files: File,
}

#[js_function(1)]
fn scan(ctx: CallContext) -> Result<JsUnknown> {
    let dir: PathBuf = ctx.get::<JsString>(0)?.into_utf8()?.as_str()?.into();

    let res = if interface::verify(&dir) {
        interface::scan(dir.clone())
    } else {
        fallback::scan(dir.clone())
    };

    match res {
        Ok(f) => ctx.env.to_js_value(&ScanResult {
            base: dir,
            files: f,
        }),
        Err(e) => Err(napi::Error::new(napi::Status::Unknown, e.to_string())),
    }
}

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
    exports.create_named_method("scan", scan)?;
    Ok(())
}
