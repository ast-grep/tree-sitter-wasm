use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_test::*;
use web_tree_sitter_sg::*;

#[wasm_bindgen_test]
async fn operator() {
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
        let predicates = query.predicates_for_pattern(1);
        let predicate = predicates[0].unchecked_ref::<PredicateResult>();
        let _operator = predicate.operator();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn operands() {
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
        let predicates = query.predicates_for_pattern(1);
        let predicate = predicates[0].unchecked_ref::<PredicateResult>();
        let _operands = predicate.operands();
        Ok(())
    }
    assert!(inner().await.is_ok());
}
