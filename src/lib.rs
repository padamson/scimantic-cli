//! Library surface for the `scimantic` CLI.
//!
//! v0.1.0 exposes only the package version. Command modules will be added
//! here as feature work lands alongside the Scimantic REST API.

/// Package name from `Cargo.toml`.
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Package version from `Cargo.toml`.
///
/// ```
/// assert_eq!(scimantic::VERSION, env!("CARGO_PKG_VERSION"));
/// ```
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
