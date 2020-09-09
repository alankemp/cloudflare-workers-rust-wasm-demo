
extern crate futures;
extern crate js_sys;
extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use js_sys::{Promise, Object};

#[macro_use]
extern crate cloudflare_workers;

use cloudflare_workers::*;

// Declare your KVs
// First parameter is the actual name of the KV, second is the name you will use in your rust code
CloudflareKV!(TestData, TestDataKV);

#[wasm_bindgen]
pub async fn request_handler() -> Result<JsValue, JsValue> {

    // debug prints will appear in your console if you `wrangler dev`
    debug_print_string("request_handler!");

    // Get a list of all the keys
    // (For some value of "all")
    let key_list = TestDataKV::list().await.unwrap();

    // Get the value for the last key in the list
    let value = TestDataKV::get(&key_list.keys.last().unwrap().name).await.unwrap();

    debug_print_string(&value);

    Ok(JsValue::from(value))
}
