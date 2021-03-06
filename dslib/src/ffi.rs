use napi::{CallContext, Error, JsString, JsUndefined, JsUnknown, Result, Status};
use std::path::PathBuf;

#[js_function(0)]
pub fn init(ctx: CallContext) -> Result<JsUndefined> {
    super::__init();
    ctx.env.get_undefined()
}

#[js_function(1)]
pub fn scan(ctx: CallContext) -> Result<JsUndefined> {
    let dir: PathBuf = ctx.get::<JsString>(0)?.into_utf8()?.as_str()?.into();

    match super::scan(dir) {
        Ok(_) => ctx.env.get_undefined(),
        Err(e) => Err(Error::new(Status::Unknown, e.to_string())),
    }
}

#[js_function(1)]
pub fn query(ctx: CallContext) -> Result<JsUnknown> {
    let dir: PathBuf = ctx.get::<JsString>(0)?.into_utf8()?.as_str()?.into();

    match super::query(dir) {
        Ok(dir) => match dir {
            Some(dir) => ctx.env.to_js_value(&dir),
            None => Ok(ctx.env.get_undefined()?.into_unknown()),
        },
        Err(e) => Err(Error::new(Status::Unknown, e.to_string())),
    }
}
