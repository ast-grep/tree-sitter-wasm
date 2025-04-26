use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
use web_tree_sitter_sg::*;

#[wasm_bindgen_test]
async fn new() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let _edit = <Edit as Default>::default();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn start_index() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let edit = <Edit as Default>::default();
        let _ = edit.start_index();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn old_end_index() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let edit = <Edit as Default>::default();
        let _ = edit.old_end_index();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn new_end_index() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let edit = <Edit as Default>::default();
        let _ = edit.new_end_index();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn start_position() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let edit = <Edit as Default>::default();
        let _ = edit.new_end_index();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn old_end_position() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let edit = <Edit as Default>::default();
        let _ = edit.new_end_index();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn new_end_position() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let edit = <Edit as Default>::default();
        let _ = edit.new_end_index();
        Ok(())
    }
    assert!(inner().await.is_ok());
}
