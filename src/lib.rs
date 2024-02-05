use wasm_bindgen::JsCast;

pub fn basic_fetch(uri: &str) -> Result<Vec<u8>, wasm_bindgen::JsValue> {
    let r = web_sys::XmlHttpRequest::new()?;
    r.open_with_async("GET", uri, false)?;
    r.override_mime_type("text/plain; charset=x-user-defined")?;
    r.send()?;
    let rs = r.response()?.dyn_into::<js_sys::JsString>()?;
    let mut v: Vec<u8> = vec![];
    for n in rs.iter() {
        v.push(n.to_le_bytes()[0]);
    }
    Ok(v)
}
