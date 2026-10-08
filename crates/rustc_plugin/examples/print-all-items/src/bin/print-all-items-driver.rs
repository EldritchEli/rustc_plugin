#![feature(rustc_private)]

use std::process::ExitCode;

fn main() -> ExitCode {
  env_logger::init();
  rustc_plugin::driver_main::<(), print_all_items::PrintAllItemsPlugin>()
}
