//! This module contains the compiler implementation.
//! Note that these modules are public for documentation
//! visiblility, But should never be used outside of the
//! module by `common` or `vm`.
//!
//! Each step in the compiler pipeline turns one datatype
//! into another. loosely, starting with `Source` (string +
//! path):
//!
//! 1. Tokens:   `lex.rs`
//! 2. Absrtact ST: `parse.rs`
//! 3. Concrete ST: `desugar.rs`
//! 4. Scoped ST:  `hoist.rs`
//! 5. Bytecode: `gen.rs`
//!
//! Note that more steps (e.g. ones applying typechecking
//! operations, optimization passes, etc.)
//! may be implemented in the future.

pub mod lex;
pub use lex::Lexer;

pub mod read;
pub use read::Reader;
// pub mod expand;
pub mod parse;
pub use parse::Parser;

pub mod desugar;
pub use desugar::Desugarer;

pub mod hoist;
pub use hoist::Hoister;
// pub mod unify;
pub mod gen;
pub use gen::Compiler;

pub mod syntax;