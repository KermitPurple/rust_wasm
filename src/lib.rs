use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_i32(s: i32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn factorial(n: i32) -> i32 {
    if n < 0 {
        panic!("n must be greater than 0")
    } else if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

#[wasm_bindgen]
pub fn show_fact(){
    for i in 0..10 {
        log_i32(i);
        log(&format!("{}! = {}", i, factorial(i)));
    }
}
