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

use wasm_bindgen::prelude::*;

pub struct TreeSitter;

impl TreeSitter {
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn init() -> Result<(), JsError> {
        Ok(())
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn init() -> Result<(), JsError> {
        web_tree_sitter::TreeSitter::init().await
    }
}
