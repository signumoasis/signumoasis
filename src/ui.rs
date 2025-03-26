pub mod backend;
pub mod components;

use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::js_sys;

#[wasm_bindgen]
pub fn sleep(ms: i32) -> js_sys::Promise {
    js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms)
            .unwrap();
    })
}

pub async fn shleep(ms: i32) {
    wasm_bindgen_futures::JsFuture::from(sleep(ms))
        .await
        .unwrap();
}
