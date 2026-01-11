use wasm_bindgen::prelude::*;
use web_tree_sitter_sg::*;

pub(crate) async fn cursor() -> Result<Option<TreeCursor>, JsValue> {
    let tree = make().await?;
    let cursor = tree.map(|tree| tree.walk());
    Ok(cursor)
}

pub(crate) async fn make() -> Result<Option<Tree>, JsValue> {
    let parser = Parser::new()?;
    let language = crate::util::language::load().await?;
    parser.set_language(Some(&language))?;
    let tree = {
        let input = <String as Default>::default().into();
        let previous_tree = Default::default();
        let options = Default::default();
        parser.parse_with_string(&input, previous_tree, options)?
    };
    Ok(tree)
}
