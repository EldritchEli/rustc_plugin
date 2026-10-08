//! A Rustc plugin that prints out the name of all items in a crate.

#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;

use std::{borrow::Cow, env, process::Command};

use rustc_hir::{
  Item,
  intravisit::{self, Visitor},
};
use rustc_middle::ty::TyCtxt;
use rustc_plugin::{
  CrateFilter, PluginResult, RustcPlugin, RustcPluginArgs, RustcWrapperType, Utf8Path,
};

// This struct is the plugin provided to the rustc_plugin framework,
// and it must be exported for use by the CLI/driver binaries.
pub struct PrintAllItemsPlugin;

impl RustcPlugin for PrintAllItemsPlugin {
  fn version(&self) -> Cow<'static, str> {
    env!("CARGO_PKG_VERSION").into()
  }

  fn driver_name(&self) -> Cow<'static, str> {
    "print-all-items-driver".into()
  }

  // In the CLI, we ask the framework to run on all crates.
  fn args(&self, _target_dir: &Utf8Path) -> RustcPluginArgs {
    RustcPluginArgs {
      args: None,
      wrapper_type: RustcWrapperType::RustcWrapper,
      rustc_enabled_for_non_filtered:
        rustc_plugin::RustcEnabledForNonFiltered::Yes,
      filter: CrateFilter::AllCrates,
      default_build_command: None,
    }
  }

  // Pass Cargo arguments (like --features) from the top-level CLI to Cargo.
  // Arguments after "--" are forwarded to the Cargo invocation.
  fn modify_cargo(&self, cargo: &mut Command, args: &Vec<String>) {
    if let Some(pos) = args.iter().position(|a| a == "--") {
      cargo.args(&args[pos + 1 ..]);
    }
  }

  // In the driver, we use the Rustc API to start a compiler session
  // for the arguments given to us by rustc_plugin.
  fn run(
    _crate_name: String,
    compiler_args: Vec<String>,
    plugin_args: &Vec<String>,
  ) -> rustc_interface::interface::Result<()> {
    let allcaps = plugin_args.iter().any(|a| a == "--allcaps" || a == "-a");
    let mut callbacks = PrintAllItemsCallbacks { allcaps };
    rustc_driver::compiler_entrypoint(&compiler_args, &mut callbacks);
    Ok(())
  }

  fn after_execution(&mut self) -> PluginResult<()> {
    Ok(())
  }
}

struct PrintAllItemsCallbacks {
  allcaps: bool,
}

impl rustc_driver::Callbacks for PrintAllItemsCallbacks {
  // At the top-level, the Rustc API uses an event-based interface for
  // accessing the compiler at different stages of compilation. In this callback,
  // all the type-checking has completed.
  fn after_analysis(
    &mut self,
    _compiler: &rustc_interface::interface::Compiler,
    tcx: TyCtxt<'_>,
  ) -> rustc_driver::Compilation {
    // We call our top-level function with access to the type context `tcx` and the CLI arguments.
    print_all_items(tcx, self.allcaps);

    // Note that you should generally allow compilation to continue. If
    // your plugin is being invoked on a dependency, then you need to ensure
    // the dependency is type-checked (its .rmeta file is emitted into target/)
    // so that its dependents can read the compiler outputs.
    rustc_driver::Compilation::Continue
  }
}

// The core of our analysis. Right now it just prints out a description of each item.
fn print_all_items(tcx: TyCtxt, allcaps: bool) {
  tcx.hir_visit_all_item_likes_in_crate(&mut PrintVisitor { allcaps, tcx });
}

struct PrintVisitor<'tcx> {
  allcaps: bool,
  tcx: TyCtxt<'tcx>,
}

impl<'tcx> Visitor<'tcx> for PrintVisitor<'tcx> {
  fn visit_item(&mut self, item: &'tcx Item<'tcx>) -> Self::Result {
    let mut msg = match item.kind.ident() {
      Some(ident) => format!(
        "There is an item \"{}\" of type \"{}\"",
        ident,
        self.tcx.def_descr(item.owner_id.to_def_id())
      ),
      None => format!(
        "There is an item of type \"{}\"",
        self.tcx.def_descr(item.owner_id.to_def_id())
      ),
    };
    if self.allcaps {
      msg = msg.to_uppercase();
    }
    println!("{msg}");

    intravisit::walk_item(self, item)
  }
}
