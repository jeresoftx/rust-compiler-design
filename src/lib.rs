#![forbid(unsafe_code)]

//! Modelos pequeños y verificables para el curso de diseño de compiladores.

pub mod ast;
pub mod bytecode;
pub mod ir;
pub mod lexer;
pub mod optimizer;
pub mod parser;
pub mod semantics;
pub mod vm;
