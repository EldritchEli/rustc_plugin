# Changelog

## v0.15.0 (`nightly-2026-10-06`)

### Breaking Changes

- **Toolchain**: Updated from `nightly-2025-08-20` to `nightly-2026-10-06` (rustc 1.101.0-nightly). Maximum supported Rust version is now **1.99**.
- **`driver_main` now returns `ExitCode`**: `driver_main()` returns `std::process::ExitCode` instead of calling `std::process::exit()` internally. Driver binaries should update their `main` function:
  ```rust
  // Before (v0.14)
  fn main() {
      rustc_plugin::driver_main::<(), MyPlugin>();
  }

  // After (v0.15)
  fn main() -> ExitCode {
      rustc_plugin::driver_main::<(), MyPlugin>()
  }
  ```
- **`rustc_utils` removed from workspace**: The `rustc_utils` crate is no longer included in the workspace build. It remains in the repository but is excluded. `rustc_plugin` does not depend on `rustc_utils`.

### Rustc API Migration

These are the upstream rustc API changes that affected this release:

| Old API | New API |
|---------|---------|
| `rustc_driver::run_compiler(&args, &mut callbacks)` | `rustc_driver::compiler_entrypoint(&args, &mut callbacks)` |
| `rustc_driver::catch_with_exit_code(f) -> i32` | `rustc_driver::catch_with_exit_code(f) -> ExitCode` |

The `Callbacks` trait (`after_crate_root_parsing`, `after_analysis`, etc.) is unchanged.

### Removed Dependencies

- `rustc_tools_util` — was unused
- `colored-print` — was unused

### Internal Cleanup

- Removed unused imports (`VersionInfo`, `DeserializeOwned`)
- Updated example plugin to match current `RustcPlugin` trait signature
