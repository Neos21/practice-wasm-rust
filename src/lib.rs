use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, practice-wasm-rust!");
}

// NOTE : 行末にセミコロンを付けないと「文」ではなく「式」となりその結果が return される
//        (`return hoge;` と `hoge` は同じ)

// JS doesn't have a chars type which means:
// - The c argument is the first char of a JS string.
// - The char returned will be a JS string.
#[wasm_bindgen]
pub fn char_example(c: char) -> char {
    // ↓ 元コード
    //'🚀'
    return c;
    // JS から文字列を渡されても最初の1文字しか受け付けなくなる
}

#[wasm_bindgen]
pub fn string_example(s: String) -> String {
    format!("Hello String {}", s)
}

// str cannot be used as a return type.
// This is because we can't return borrowed references with the wasm_bindgen macro.
#[wasm_bindgen]
pub fn str_example(s: &str) -> String {
    format!("Hello str {}", s)
}

// assume the same for u32, usize, etc.
#[wasm_bindgen]
pub fn number_example(n: i32) -> i32 {
    n + 100
}

#[wasm_bindgen]
pub fn bool_example(b: bool) -> bool {
    // ↓ 元コード
    //true
    return b;
    // JS における若干の曖昧型変換がなされる模様
}

// `Box<[JsValue]>` are the representation for a JS array object.
// When it comes to Js Arrays:
// - They are iterable.
// - Can contain multiple types by being of type JsValue (strictly typed arrays exist for numbers).
// - Don't really support N-dimensional arrays and are expensive to work with.
#[wasm_bindgen]
pub fn mixed_array_example(array: Box<[JsValue]>) -> Box<[JsValue]> {
    for _value in array.iter() {
        // compute things...
    }
    
    vec![
        "Hello".into(),
        512.into(),
        JsValue::NULL,
        JsValue::UNDEFINED,
        61.20.into()
    ].into_boxed_slice()
}

// Typed arrays are only available for number types.
// For example, the function below will return a JS Int32Array type.
#[wasm_bindgen]
pub fn typed_array_example(_array: Box<[i32]>) -> Box<[i32]> {
    vec![1, 2, 3, 4, 5, 6, 7].into_boxed_slice()
}

// When it comes to Option:
// - Some returns the value inside.
// - None returns a JS undefined.
#[wasm_bindgen(catch)]
pub fn option_example() -> Option<i32> {
    None
}

// When it comes to Result
// - Result<T, JsValue> is the only supported signature. T must be convertible to a JsValue.
// - #[wasm_bindgen(catch)] must be used when returning a result.
// - Err will be equivalent to a JS thrown error.
// - Ok will return the value inside.
#[wasm_bindgen]
pub fn result_example() -> Result<i32, JsValue> {
    // With the wasm prelude imported, we can convert most common types by calling .into()
    Err("Look Pa, I'm throwing a JS error!".into())
}
