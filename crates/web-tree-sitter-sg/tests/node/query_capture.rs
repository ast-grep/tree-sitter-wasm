use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_test::*;
use web_tree_sitter_sg::*;

#[wasm_bindgen_test]
async fn name() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let (parser, _, query) = crate::util::language::query().await?;
        let tree = {
            let input = "function one() { two(); function three() {} }".into();
            let previous_tree = Default::default();
            let options = Default::default();
            parser.parse_with_string(&input, previous_tree, options)?
        }
        .unwrap();
        let captures = {
            let node = tree.root_node();
            let start_position = Default::default();
            let end_position = Default::default();
            query.captures(&node, start_position, end_position)
        };
        let capture = captures[0].unchecked_ref::<QueryCapture>();
        let _name = capture.name();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn node() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let (parser, _, query) = crate::util::language::query().await?;
        let tree = {
            let input = "function one() { two(); function three() {} }".into();
            let previous_tree = Default::default();
            let options = Default::default();
            parser.parse_with_string(&input, previous_tree, options)?
        }
        .unwrap();
        let captures = {
            let node = tree.root_node();
            let start_position = Default::default();
            let end_position = Default::default();
            query.captures(&node, start_position, end_position)
        };
        let capture = captures[0].unchecked_ref::<QueryCapture>();
        let _node = capture.node();
        Ok(())
    }
    assert!(inner().await.is_ok());
}
