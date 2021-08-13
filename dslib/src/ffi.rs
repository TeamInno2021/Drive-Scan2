use napi::{CallContext, Error, JsString, JsUndefined, JsUnknown, Result, Status};
use std::path::PathBuf;

#[js_function(0)]
pub fn init(ctx: CallContext) -> Result<JsUndefined> {
    super::__init();
    ctx.env.get_undefined()
}

#[js_function(1)]
pub fn scan(ctx: CallContext) -> Result<JsUnknown> {
    let dir: PathBuf = ctx.get::<JsString>(0)?.into_utf8()?.as_str()?.into();

    match super::scan(dir) {
        Ok(f) => ctx.env.to_js_value(&f),
        Err(e) => Err(Error::new(Status::Unknown, e.to_string())),
    }
}

#[js_function(1)]
pub fn query(ctx: CallContext) -> Result<JsUnknown> {
    let dir: PathBuf = ctx.get::<JsString>(0)?.into_utf8()?.as_str()?.into();

    match super::query(dir) {
        Ok(dir) => ctx.env.to_js_value(&dir),
        Err(e) => Err(Error::new(Status::Unknown, e.to_string())),
    }
}
