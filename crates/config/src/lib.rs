//! # config
//!
//! Loads and validates router configuration from a TOML file.
//!
//! ## Structure
//! ```toml
//! [router]
//! bind = "0.0.0.0:8080"
//!
//! [[routes]]
//! path   = "/api"
//! plugin = "plugins/auth.wasm"
//! upstream = "http://backend:3000"
//!
//! [stellar]
//! horizon_url = "https://horizon-testnet.stellar.org"
//! network_passphrase = "Test SDF Network ; September 2015"
//! ```

use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::path::Path;

// ---------------------------------------------------------------------------
// Top-level config
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouterConfig {
    pub router: RouterSection,
    #[serde(default)]
    pub routes: Vec<RouteConfig>,
    #[serde(default)]
    pub stellar: Option<StellarConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouterSection {
    /// Socket address to bind, e.g. "0.0.0.0:8080"
    pub bind: String,
    /// Maximum number of concurrent connections (default: 1024)
    #[serde(default = "default_max_connections")]
    pub max_connections: usize,
}

fn default_max_connections() -> usize { 1024 }

// ---------------------------------------------------------------------------
// Route config
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteConfig {
    /// URL path prefix this route matches, e.g. "/api"
    pub path: String,
    /// Path to the `.wasm` plugin file to run for this route (optional)
    pub plugin: Option<String>,
    /// Upstream origin to proxy to after plugin processing
    pub upstream: String,
}

// ---------------------------------------------------------------------------
// Stellar config
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StellarConfig {
    /// Horizon REST API base URL
    pub horizon_url: String,
    /// Stellar network passphrase (mainnet or testnet)
    pub network_passphrase: String,
    /// Optional Soroban RPC endpoint
    pub soroban_rpc_url: Option<String>,
}

// ---------------------------------------------------------------------------
// Loader
// ---------------------------------------------------------------------------

impl RouterConfig {
    /// Load config from a TOML file at `path`.
    ///
    /// # TODO
    /// - [ ] Support environment-variable overrides (e.g. `ROUTER_BIND`)
    /// - [ ] Hot-reload via `inotify` / `kqueue`
    pub fn from_file(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let raw = std::fs::read_to_string(path.as_ref())
            .with_context(|| format!("reading config {:?}", path.as_ref()))?;
        let cfg: Self = toml::from_str(&raw).context("parsing config TOML")?;
        cfg.validate()?;
        Ok(cfg)
    }

    fn validate(&self) -> anyhow::Result<()> {
        // TODO: validate bind address parses as SocketAddr
        // TODO: validate each route's upstream is a valid URL
        // TODO: validate plugin paths exist on disk
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_minimal_config() {
        let toml = r#"
            [router]
            bind = "127.0.0.1:8080"

            [[routes]]
            path     = "/"
            upstream = "http://localhost:3000"
        "#;
        let cfg: RouterConfig = toml::from_str(toml).unwrap();
        assert_eq!(cfg.router.bind, "127.0.0.1:8080");
        assert_eq!(cfg.routes[0].path, "/");
    }
}
