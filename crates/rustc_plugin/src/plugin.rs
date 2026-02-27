use std::{borrow::Cow, path::PathBuf, process::Command};

use cargo_metadata::camino::Utf8Path;
use serde::{Serialize, de::DeserializeOwned};

use crate::{PluginResult, build_commands::CargoBuildCommand, driver};

pub enum RustcWrapperType {
  RustcWrapper,
  RustcWorkspaceWrapper,
}
impl RustcWrapperType {
  pub fn as_env_var(&self) -> String {
    match self {
      RustcWrapperType::RustcWrapper => "RUSTC_WRAPPER".into(),
      RustcWrapperType::RustcWorkspaceWrapper => "RUSTC_WORKSPACE_WRAPPER".into(),
    }
  }
}

/// Specification of a set of crates.
pub enum CrateFilter {
  /// Every crate in the workspace and all transitive dependencies.
  AllCrates,

  /// Just crates in the workspace.
  OnlyWorkspace,

  /// Only the crate containing a specific file.
  CrateContainingFile(PathBuf),

  RunOnCrates(Vec<String>),
}

/// Arguments from your plugin to the rustc_plugin framework.
pub struct RustcPluginArgs<Args> {
  /// Whatever CLI arguments you want to pass along.
  pub args: Args,
  /// Should we run or driver as a RUSTC_WRAPPER or a RUSTC_WORKSPACE_WRAPPER?
  pub wrapper_type: RustcWrapperType,
  pub rustc_enabled_for_non_filtered: bool,
  /// Which crates you want to run the plugin on.
  pub filter: CrateFilter,
  pub default_build_command: Option<CargoBuildCommand>,
}

pub trait RustcPlugin<T = ()>: Sized {
  /// Command-line arguments passed by the user.
  type Args: Serialize + DeserializeOwned;
  /// Returns the version of your plugin.
  ///
  /// A sensible default is your plugin's Cargo version:
  ///
  /// ```ignore
  /// env!("CARGO_PKG_VERSION").into()
  ///
  ///
  ///
  fn version(&self) -> Cow<'static, str>;

  /// Returns the name of your driver binary as it's installed in the filesystem.
  ///
  /// Should be just the filename, not the full path.
  fn driver_name(&self) -> Cow<'static, str>;

  /// Parses and returns the CLI arguments for the plugin.
  fn args(&self, target_dir: &Utf8Path) -> RustcPluginArgs<Self::Args>;
  /// Optionally modify the `cargo` command that launches rustc.
  /// For example, you could pass a `--feature` flag here.
  fn modify_cargo(&self, _cargo: &mut Command, _args: &Self::Args) {}
  /// Executes the plugin with a set of compiler and plugin args.
  fn run(
    compiler_args: Vec<String>,
    plugin_args: Self::Args,
  ) -> rustc_interface::interface::Result<()>;

  fn driver_main() {
    driver::driver_main::<T, Self>();
  }

  ///executes right before the main cargo execution
  fn before_execution(&mut self) {}
  ///executes right after main cargo execution has finished. The return value used in this method is what is returned in `cli_main`
  fn after_execution(&self) -> PluginResult<T>;
}

/// The name of the environment variable shared between the CLI and the driver.
/// Must not conflict with any other env var used by Cargo.
pub const PLUGIN_ARGS: &str = "PLUGIN_ARGS";
