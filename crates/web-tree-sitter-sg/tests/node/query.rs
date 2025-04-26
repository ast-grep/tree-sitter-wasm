use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
use web_tree_sitter_sg::*;

#[wasm_bindgen_test]
async fn capture_names() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let (_, _, query) = crate::util::language::query().await?;
        assert_eq!(
            vec!["fn-def".into(), "fn-ref".into()].into_boxed_slice(),
            query.capture_names()
        );
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn delete() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let (_, _, query) = crate::util::language::query().await?;
        query.delete();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn matches() {
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
        let _matches = {
            let node = tree.root_node();
            let start_position = Default::default();
            let end_position = Default::default();
            query.matches(&node, start_position, end_position)
        };
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn captures() {
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
        let _captures = {
            let node = tree.root_node();
            let start_position = Default::default();
            let end_position = Default::default();
            query.captures(&node, start_position, end_position)
        };
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn predicates_for_pattern() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let parser = Parser::new()?;
        let language = crate::util::language::load().await?;
        parser.set_language(Some(&language))?;
        let query = r###"
        ((binary_expression
            left: (identifier) @a
            right: (identifier) @b)
        (#something? @a @b)
            (#match? @a "c")
            (#something-else? @a "A" @b "B"))
        ((identifier) @c
           (#hello! @c))
        "if" @d
        "###
        .into();
        let query = language.query(&query)?;
        // FIXME: check output
        let _predicates = query.predicates_for_pattern(0);
        Ok(())
    }
    assert!(inner().await.is_ok());
}
