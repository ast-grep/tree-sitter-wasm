use std::{error::Error, path::PathBuf};
use wasm_bindgen::prelude::*;
use web_tree_sitter_sg::*;

pub(crate) async fn load() -> Result<Language, JsValue> {
    let id = "javascript";
    let url = crate::util::language::url(id);
    let url = url.unwrap_or_else(|_| panic!("failed to obtain url for language '{}'", id));
    let language = Language::load_path(&url).await?;
    Ok(language)
}

pub(crate) async fn query() -> Result<(Parser, Language, Query), JsValue> {
    let parser = Parser::new()?;
    let language = crate::util::language::load().await?;
    parser.set_language(Some(&language))?;
    let query = r###"
    (function_declaration name: (identifier) @fn-def)
    (call_expression function: (identifier) @fn-ref)
    "###
    .into();
    let query = language.query(&query)?;
    Ok((parser, language, query))
}

pub(crate) fn url(language: &str) -> Result<String, Box<dyn Error>> {
    let mut path = PathBuf::new();
    path.push(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push(format!("tree-sitter-{}.wasm", language));
    let path = match path.into_os_string().into_string() {
        Ok(path) => Ok::<_, Box<dyn Error>>(path),
        Err(err) => Err(format!("failed to convert path to string `{:?}`", err).into()),
    }?;
    Ok(super::require.resolve(&path, None))
}
