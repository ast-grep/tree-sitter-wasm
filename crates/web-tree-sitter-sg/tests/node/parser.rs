#![allow(unused)]

use js_sys::Function;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_test::*;
use web_tree_sitter_sg::*;

#[wasm_bindgen_test]
async fn new() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let _parser = Parser::new()?;
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn delete() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let parser = Parser::new()?;
        parser.delete();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn parse_with_function() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let parser = Parser::new()?;
        let language = crate::util::language::load().await?;
        parser.set_language(Some(&language))?;
        let clo = Closure::wrap(Box::new(move |_, _, _| None) as Box<InputClosureType>);
        let _tree = {
            let input = clo.as_ref().unchecked_ref::<Function>();
            let previous_tree = Default::default();
            let options = Default::default();
            parser.parse_with_function(input, previous_tree, options)
        };
        clo.forget();
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn parse_with_string() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let parser = Parser::new()?;
        let language = crate::util::language::load().await?;
        parser.set_language(Some(&language))?;
        let _tree = {
            let input = String::new().into();
            let previous_tree = Default::default();
            let options = Default::default();
            parser.parse_with_string(&input, previous_tree, options)
        };
        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn reset() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let parser = Parser::new()?;
        parser.reset();
        Ok(())
    }
    assert!(inner().await.is_ok())
}

#[wasm_bindgen_test]
async fn set_get_language() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let parser = Parser::new()?;
        let language = crate::util::language::load().await?;

        parser.set_language(Some(&language))?;
        assert_eq!(Some(language), parser.language());

        parser.set_language(None)?;
        assert_eq!(None, parser.language());

        Ok(())
    }
    assert!(inner().await.is_ok());
}

#[wasm_bindgen_test]
async fn set_get_logger() {
    async fn inner() -> Result<(), JsValue> {
        TreeSitter::init(None).await?;
        let parser = Parser::new()?;
        let language = crate::util::language::load().await?;
        parser.set_language(Some(&language))?;

        let clo = Closure::wrap(Box::new(move |_, _, _| {}) as Box<LoggerClosureType>);
        let logger = clo.as_ref().unchecked_ref::<Function>();
        parser.set_logger(Some(logger));
        assert_eq!(Some(logger), parser.get_logger().as_ref());

        parser.set_logger(None);
        assert_eq!(None, parser.get_logger().as_ref());

        Ok(())
    }
    assert!(inner().await.is_ok());
}
