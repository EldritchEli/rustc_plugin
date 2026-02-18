use std::str::FromStr;

/// Interface between your plugin and the rustc_plugin framework.
/*    4.2.1. cargo bench
4.2.2. cargo build
4.2.3. cargo check
4.2.4. cargo clean
4.2.5. cargo clippy
4.2.6. cargo doc
4.2.7. cargo fetch
4.2.8. cargo fix
4.2.9. cargo fmt
4.2.10. cargo miri
4.2.11. cargo report
4.2.12. cargo run
4.2.13. cargo rustc
4.2.14. cargo rustdoc
4.2.15. cargo test */
#[derive(Debug, Clone, Copy)]
pub enum CargoBuildCommand {
  Build,
  Check,
  Clean,
  Clippy,
  Doc,
  Fetch,
  Fix,
  Fmt,
  Miri,
  Report,
  Run,
  RustC,
  Rustdoc,
  Test,
}
#[derive(Clone, Debug)]
pub struct ParseCargoError {
  inner: String,
}
impl From<&str> for ParseCargoError {
  fn from(value: &str) -> Self {
    Self {
      inner: value.to_string(),
    }
  }
}
impl FromStr for CargoBuildCommand {
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let res = match s {
      "build" => Self::Build,
      "check" => Self::Check,
      "clean" => Self::Clean,
      "clippy" => Self::Clippy,
      "doc" => Self::Doc,
      "fetch" => Self::Fetch,
      "fix" => Self::Fix,
      "fmt" => Self::Fmt,
      "miri" => Self::Miri,
      "run" => Self::Run,
      "rustc" => Self::RustC,
      "rustdoc" => Self::Rustdoc,
      "test" => Self::Test,
      _ => return Err("failed to parse build command".into()),
    };
    Ok(res)
  }

  type Err = ParseCargoError;
}

impl From<CargoBuildCommand> for String {
  fn from(value: CargoBuildCommand) -> Self {
    match value {
      CargoBuildCommand::Build => "build".into(),
      CargoBuildCommand::Check => "check".into(),
      CargoBuildCommand::Clean => "clean".into(),
      CargoBuildCommand::Clippy => "clippy".into(),
      CargoBuildCommand::Doc => "doc".into(),
      CargoBuildCommand::Fetch => "fetch".into(),
      CargoBuildCommand::Fix => "fix".into(),
      CargoBuildCommand::Fmt => "fmt".into(),
      CargoBuildCommand::Miri => "miri".into(),
      CargoBuildCommand::Report => "report".into(),
      CargoBuildCommand::Run => "run".into(),
      CargoBuildCommand::RustC => "rustc".into(),
      CargoBuildCommand::Rustdoc => "rustdoc".into(),
      CargoBuildCommand::Test => "test".into(),
    }
  }
}
