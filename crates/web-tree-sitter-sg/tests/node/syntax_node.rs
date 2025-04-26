use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
use web_tree_sitter_sg::*;

#[wasm_bindgen_test]
async fn id() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let _ = node.id();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn tree() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let _ = node.tree();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn type_() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let _ = node.type_();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn type_id() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let _ = node.type_id();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn text() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let _ = node.text();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn start_position() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let _ = node.start_position();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn end_position() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let _ = node.end_position();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn start_index() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let _ = node.start_index();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn end_index() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let _ = node.end_index();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn parent() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let _ = node.parent();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn children() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let _ = node.children();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn named_children() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let _ = node.named_children();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn child_count() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let _ = node.child_count();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn named_child_count() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let _ = node.named_child_count();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn first_child() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let _ = node.first_child();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn first_named_child() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let _ = node.first_named_child();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn last_child() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let _ = node.last_child();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn last_named_child() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let _ = node.last_named_child();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn next_sibling() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let _ = node.next_sibling();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn next_named_sibling() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let _ = node.next_named_sibling();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn previous_sibling() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let _ = node.previous_sibling();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn previous_named_sibling() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let _ = node.previous_named_sibling();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn has_changes() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let _ = node.has_changes();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn has_error() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let _ = node.has_error();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn is_error() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let _ = node.is_error();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn equals() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        assert!(node.equals(&node));
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn is_missing() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let _ = node.is_missing();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn is_named() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let _ = node.is_named();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn to_string() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let _ = node.to_string();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn child() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let field_id = Default::default();
        let _ = node.child(field_id);
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn named_child() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let field_id = Default::default();
        let _ = node.named_child(field_id);
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn child_for_field_id() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let field_id = Default::default();
        let _ = node.child_for_field_id(field_id);
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn child_for_field_name() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let field_name = Default::default();
        let _ = node.child_for_field_name(field_name);
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn child_with_descendant() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let descendant = crate::util::syntax_node::make().await?.unwrap();
        let _ = node.child_with_descendant(&descendant);
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn descendant_for_index() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let index = Default::default();
        let _ = node.descendant_for_index(index);
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn descendant_for_index_range() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let start_index = Default::default();
        let end_index = Default::default();
        let _ = node.descendant_for_index_range(start_index, end_index);
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn descendant_of_type_string() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let type_ = Default::default();
        let start_position = Default::default();
        let end_position = Default::default();
        let _ = node.descendants_of_type_string(type_, start_position, end_position);
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn descendant_of_type_array() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let type_ = Default::default();
        let start_position = Default::default();
        let end_position = Default::default();
        let _ = node.descendants_of_type_array(type_, start_position, end_position);
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn named_descendant_for_index() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let index = Default::default();
        let _ = node.named_descendant_for_index(index);
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn named_descendant_for_index_range() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let start_index = Default::default();
        let end_index = Default::default();
        let _ = node.named_descendant_for_index_range(start_index, end_index);
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn descendant_for_position() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let position = Default::default();
        let _ = node.descendant_for_position(&position);
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn descendant_for_position_range() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let start_position = Default::default();
        let end_position = Default::default();
        let _node = node.descendant_for_position_range(&start_position, &end_position);
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn named_descendant_for_position() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let position = Default::default();
        let _node = node.named_descendant_for_position(&position);
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn named_descendant_for_position_range() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let start_position = Default::default();
        let end_position = Default::default();
        let _node = node.named_descendant_for_position_range(&start_position, &end_position);
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn walk() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let node = crate::util::syntax_node::make().await?.unwrap();
        let _cursor = node.walk();
        Ok(())
    }
    assert!(inner().await.is_ok());
}
