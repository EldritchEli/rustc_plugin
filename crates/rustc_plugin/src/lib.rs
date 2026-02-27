//! A framework for writing plugins that integrate with the Rust compiler.
//!
//! Much of this library is either directly copy/pasted, or otherwise generalized
//! from the Clippy driver: <https://github.com/rust-lang/rust-clippy/tree/master/src>

#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_session;
extern crate rustc_span;
use std::io;

pub use build::build_main;
pub use cargo_metadata::camino::Utf8Path;
pub use cli::cli_main;
pub use driver::driver_main;
pub use plugin::{CrateFilter, RustcPlugin, RustcPluginArgs, RustcWrapperType};
#[doc(hidden)]
pub use thiserror::Error;

/// The toolchain channel that this version of rustc_plugin was built with.
///
/// For example, `nightly-2025-08-20`
pub const CHANNEL: &str = env!("RUSTC_CHANNEL");

mod build;
mod build_commands;
mod cli;
mod driver;
mod plugin;

#[derive(Error, Debug)]
pub enum RustcPluginError {
  #[error("got io error: {0}")]
  IoError(#[from] io::Error),
  #[error("error when running driver: {0}")]
  DriverError(String),
  #[error("got exit code {0}")]
  ExitCode(i32),
  #[error("Failed execution of return function. got {0}")]
  ClientReturnError(String),
}

pub type PluginResult<T> = std::result::Result<T, RustcPluginError>;
