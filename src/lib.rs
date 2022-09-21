mod utils;

use core::panic;

use js_sys::{Array, Function};
use serde::{Deserialize, Serialize};
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
// Note that SomeStruct must not implement the Copy trait, as in the not-minimal-example I have Vec<>s in the struct
pub struct SomeStruct {
    pub(crate) field_to_be_modified: i32,
}

#[wasm_bindgen]
impl SomeStruct {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        set_panic_hook();

        Self {
            field_to_be_modified: 0,
        }
    }

    pub fn modify_field(&mut self, value: i32) {
        self.field_to_be_modified = value;
    }

    pub fn field(&self) -> i32 {
        self.field_to_be_modified
    }

    #[wasm_bindgen]
    pub async fn with_callback(&self, function_or_promise: JsValue) -> Result<JsValue, JsValue> {
        let mut s = SomeStruct::new();
        let function = function_or_promise.dyn_into::<Function>().map_err(|_| {
            JsError::new("The provided callback is not a function. Please provide a function.")
        })?;

        run_any_function(&mut s, function, vec![JsValue::from(1u32)]).await
    }
}

pub(crate) async fn run_any_function(
    ax: &mut SomeStruct,
    function_or_promise: js_sys::Function,
    arguments: Vec<JsValue>,
) -> Result<JsValue, JsValue> {
    let result = run_function(ax, function_or_promise, arguments)?;

    // Handle functions defined like "async function(args) {}"
    if result.has_type::<js_sys::Promise>() {
        return run_promise(result).await;
    } else {
        Ok(result)
    }
}

async fn run_promise(promise_arg: JsValue) -> Result<JsValue, JsValue> {
    let promise = js_sys::Promise::from(promise_arg);
    let future = JsFuture::from(promise);
    future.await
}

fn run_function(
    my_struct: &mut SomeStruct,
    function: js_sys::Function,
    arguments: Vec<JsValue>,
) -> Result<JsValue, JsValue> {
    let args = Array::new();

    // This is the reason modifications from JS aren't reflected in Rust, but without it JsValue::from doesn't work
    let clone = my_struct.clone();

    // my_struct is the first function argument
    // TODO: JsValue::from only works when cloned, not on the original struct. Why?
    // Best would be directly passing my_struct, as then modifications would work
    // Passing a pointer to the struct would also be fine, as long as methods can be called on it from JavaScript
    args.push(&JsValue::from(clone));

    for arg in arguments {
        args.push(&arg);
    }

    // Actually call the function
    let result = function.apply(&JsValue::NULL, &args)?;

    // TODO: How to turn result back into a SomeStruct struct?

    // Copying fields manually also doesn't work because of borrow checker:
    // my_struct.field_to_be_modified = clone.field_to_be_modified;

    Ok(result)
}
