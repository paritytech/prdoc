//! This is the main crate of the [prdoc](/prdoc) cli.
#![warn(missing_docs)]

pub mod commands;
pub mod common;
pub mod config;

pub mod doc_filename;
pub mod docfile;
pub mod docfile_wrapper;
pub mod prdoc_source;

pub mod error;
pub mod schema;
pub mod title;
pub mod utils;
