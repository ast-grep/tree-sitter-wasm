#[cfg(not(target_arch = "wasm32"))]
mod native {
    use crate::{error::QueryError, language::Language};

    pub struct Query {
        pub inner: tree_sitter::Query,
    }

    impl Query {
        #[inline]
        pub fn new(language: &Language, source: &str) -> Result<Self, QueryError> {
            let inner = tree_sitter::Query::new(&language.inner, source)?;
            Ok(Self { inner })
        }
    }

    impl std::fmt::Debug for Query {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            std::fmt::Debug::fmt(&self.inner, fmt)
        }
    }

    impl From<tree_sitter::Query> for Query {
        #[inline]
        fn from(inner: tree_sitter::Query) -> Self {
            Self { inner }
        }
    }

    impl std::panic::RefUnwindSafe for Query {
    }

    unsafe impl Send for Query {
    }

    unsafe impl Sync for Query {
    }

    impl Unpin for Query {
    }

    impl std::panic::UnwindSafe for Query {
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[cfg(target_arch = "wasm32")]
mod wasm {
    use crate::{error::QueryError, language::Language};

    pub struct Query {
        pub(crate) inner: web_tree_sitter_sg::Query,
    }

    impl Query {
        #[inline]
        pub fn new(language: &Language, source: &str) -> Result<Self, QueryError> {
            let inner = language.inner.query(&source.into())?;
            Ok(Self { inner })
        }
    }

    impl std::fmt::Debug for Query {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            std::fmt::Debug::fmt(&self.inner, fmt)
        }
    }

    impl Drop for Query {
        #[inline]
        fn drop(&mut self) {
            self.inner.delete();
        }
    }

    impl From<web_tree_sitter_sg::Query> for Query {
        #[inline]
        fn from(inner: web_tree_sitter_sg::Query) -> Self {
            Self { inner }
        }
    }

    impl std::panic::RefUnwindSafe for Query {
    }

    unsafe impl Send for Query {
    }

    unsafe impl Sync for Query {
    }

    impl Unpin for Query {
    }

    impl std::panic::UnwindSafe for Query {
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm::*;
