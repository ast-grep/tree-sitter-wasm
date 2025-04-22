use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
use web_tree_sitter_sg::*;

#[wasm_bindgen_test]
async fn new() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let _point = <Point as Default>::default();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn row() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let point = <Point as Default>::default();
        let _row = point.row();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn column() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let point = <Point as Default>::default();
        let _column = point.column();
        Ok(())
    }
    assert!(inner().await.is_ok());
}
