#![feature(box_syntax)]
#![feature(box_patterns)]
#![feature(const_fn)] // For the `transmute` in `P::new`
#![feature(const_fn_transmute)]
#![feature(const_panic)]
#![feature(crate_visibility_modifier)]
#![feature(label_break_value)]
#![feature(nll)]
#![feature(or_patterns)]
#![recursion_limit = "256"]

#[macro_use]
extern crate rustc_macros;

pub mod ast;
pub mod ptr;
pub mod token;
pub mod tokenstream;
pub mod visit;

pub use ast::*;

use rustc_data_structures::stable_hasher::{HashStable, StableHasher};

pub use tscore_span::HashStableContext;
