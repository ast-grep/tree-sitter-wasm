use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
use web_tree_sitter_sg::*;

#[wasm_bindgen_test]
async fn new() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let _options = <ParseOptions as Default>::default();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn included_ranges() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let options = <ParseOptions as Default>::default();
        let _included_ranges = options.included_ranges();
        Ok(())
    }
    assert!(inner().await.is_ok());
}
