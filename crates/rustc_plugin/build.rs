const DEFAULT_CHANNEL: &str = "nightly-2025-08-20";

fn main() {
  let toolchain_toml = include_str!("rust-toolchain.toml");
  let Ok(toolchain_table) = toolchain_toml.parse::<toml::Table>() else {
    log::error!("couldn't parse toolchain_toml, falling back to {DEFAULT_CHANNEL}");
    println!("cargo:rustc-env=RUSTC_CHANNEL={DEFAULT_CHANNEL}");
    return;
  };

  let toolchain = toolchain_table["toolchain"]
    .as_table()
    .expect("expected rust-toolchain.toml to contain a valid toolchain")
    .clone();
  let channel = toolchain["channel"]
    .as_str()
    .expect("rust-toolchain.toml should contain channel field");

  println!("cargo:rustc-env=RUSTC_CHANNEL={channel}");
}
