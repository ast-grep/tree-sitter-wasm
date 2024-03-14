// @ts-check
"use-strict";

// this is a silly reexport because wasm-pack cannot handle
// `const Parser = require('web-tree-sitter')`
// nor web-tree-sitter behave as a good export citizen
const TreeSitter = require("web-tree-sitter");
module.exports = {
  Parser: TreeSitter,
  Language: TreeSitter.Language,
  init: TreeSitter.init,
};
