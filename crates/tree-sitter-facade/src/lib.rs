mod error;
mod input_edit;
mod language;
mod logger;
mod node;
mod parser;
mod point;
mod query;
mod range;
mod tree;
mod tree_cursor;

pub use error::*;
pub use input_edit::*;
pub use language::*;
pub use logger::*;
pub use node::*;
pub use parser::*;
pub use point::*;
pub use query::*;
pub use range::*;
pub use tree::*;
pub use tree_cursor::*;
// #[cfg(not(target_arch = "wasm32"))]
// pub use tree_sitter::{
//     WasmErrorKind, WasmError, WasmStore, wasmtime,
//     wasm_engine_t, wasm_stdlib_symbols,
// };

use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
pub use js_sys::Object;

pub struct TreeSitter;

impl TreeSitter {
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn init() -> Result<(), JsError> {
        Ok(())
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn init(options: Option<Object>) -> Result<(), JsError> {
        web_tree_sitter_sg::TreeSitter::init(options).await
    }
}
