use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
use web_tree_sitter_sg::*;

#[wasm_bindgen_test]
async fn new() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let _range = <Range as Default>::default();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn start_position() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let range = <Range as Default>::default();
        let _start_position = range.start_position();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn end_position() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let range = <Range as Default>::default();
        let _end_position = range.end_position();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn start_index() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let range = <Range as Default>::default();
        let _start_index = range.start_index();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn end_index() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let range = <Range as Default>::default();
        let _end_index = range.end_index();
        Ok(())
    }
    assert!(inner().await.is_ok());
}
