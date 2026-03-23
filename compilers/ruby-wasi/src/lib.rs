#![warn(missing_docs)]

use ruby::Ruby;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct RubyWASI {
    runtime: Ruby,
}

#[wasm_bindgen]
impl RubyWASI {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let runtime = Ruby::new().unwrap();
        Self { runtime }
    }

    #[wasm_bindgen]
    pub fn evaluate(&mut self, code: &str) -> String {
        match self.runtime.execute_script(code) {
            Ok(_) => "Execution successful".to_string(),
            Err(error) => format!("Error: {:?}", error),
        }
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.runtime = Ruby::new().unwrap();
    }
}
