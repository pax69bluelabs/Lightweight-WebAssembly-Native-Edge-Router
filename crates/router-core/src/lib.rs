//! # router-core
//!
//! Core HTTP routing engine. Accepts incoming connections, matches routes from
//! config, invokes WASM plugins via `wasm-runtime`, and proxies to upstreams.
//!
//! ## TODO
//! - [ ] Implement `Router::run` — bind TCP, accept connections, dispatch
//! - [ ] Implement route matching (longest-prefix)
//! - [ ] Integrate `PluginRegistry` from `wasm-runtime`
//! - [ ] Proxy matched requests to upstream with hyper

pub use config::RouterConfig;
