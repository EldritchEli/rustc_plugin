//! A framework for writing plugins that integrate with the Rust compiler.
//!
//! Much of this library is either directly copy/pasted, or otherwise generalized
//! from the Clippy driver: <https://github.com/rust-lang/rust-clippy/tree/master/src>

#![feature(rustc_private)]

extern crate rustc_ast;
extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_session;
extern crate rustc_span;
use std::io;

pub use build::build_main;
pub use cargo_metadata::camino::Utf8Path;
pub use cli::cli_main;
pub use driver::driver_main;
pub use plugin::{
  CrateFilter, RustcEnabledForNonFiltered, RustcPlugin, RustcPluginArgs, RustcWrapperType,
};
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

pub use build_commands::{CargoBuildCommand, ParseCargoError};
pub use plugin::DefaultBuildCommand;
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

pub struct EmptyCallbacks;
impl rustc_driver::Callbacks for EmptyCallbacks {
  fn after_crate_root_parsing(
    &mut self,
    _compiler: &rustc_interface::interface::Compiler,
    _krate: &mut rustc_ast::Crate,
  ) -> rustc_driver::Compilation {
    rustc_driver::Compilation::Stop
  }
}
