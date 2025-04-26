use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
use web_tree_sitter_sg::*;

#[wasm_bindgen_test]
async fn node_type() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let cursor = crate::util::tree::cursor().await?.unwrap();
        cursor.node_type();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn node_text() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let cursor = crate::util::tree::cursor().await?.unwrap();
        cursor.node_text();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn node_is_named() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let cursor = crate::util::tree::cursor().await?.unwrap();
        cursor.node_is_named();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn start_position() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let cursor = crate::util::tree::cursor().await?.unwrap();
        cursor.start_position();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn end_position() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let cursor = crate::util::tree::cursor().await?.unwrap();
        cursor.end_position();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn start_index() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let cursor = crate::util::tree::cursor().await?.unwrap();
        cursor.start_index();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn end_index() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let cursor = crate::util::tree::cursor().await?.unwrap();
        cursor.end_index();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn reset() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let cursor = crate::util::tree::cursor().await?.unwrap();
        let node = cursor.current_node();
        cursor.reset(&node);
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn delete() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let cursor = crate::util::tree::cursor().await?.unwrap();
        cursor.delete();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn current_node() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let cursor = crate::util::tree::cursor().await?.unwrap();
        cursor.current_node();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn current_field_id() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let cursor = crate::util::tree::cursor().await?.unwrap();
        cursor.current_field_id();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn current_field_name() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let cursor = crate::util::tree::cursor().await?.unwrap();
        cursor.current_field_name();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn goto_parent() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let cursor = crate::util::tree::cursor().await?.unwrap();
        cursor.goto_parent();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn goto_first_child() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let cursor = crate::util::tree::cursor().await?.unwrap();
        cursor.goto_first_child();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn goto_next_sibling() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let cursor = crate::util::tree::cursor().await?.unwrap();
        cursor.goto_next_sibling();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn goto_previous_sibling() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let cursor = crate::util::tree::cursor().await?.unwrap();
        cursor.goto_previous_sibling();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn goto_first_child_for_index() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let cursor = crate::util::tree::cursor().await?.unwrap();
        cursor.goto_first_child_for_index(0);
        Ok(())
    }
    assert!(inner().await.is_ok());
}
