//! # wasm-runtime
//!
//! Loads `.wasm` plugin files and executes them against HTTP requests using
//! [Wasmtime](https://wasmtime.dev/).
//!
//! ## Design (inspired by proxy-wasm / Envoy WASM filter)
//!
//! Each plugin is a sandboxed Wasmtime `Instance`.  The host exposes a small
//! set of imported functions (`host_log`, `host_kv_get`, …) and calls the
//! plugin's exported `on_request` function for every matching request.
//!
//! Memory is shared via a single linear-memory region: the host writes the
//! serialised `Request` into the plugin's memory, calls `on_request(ptr, len)`,
//! and reads back the `Action` from the returned `(ptr << 32 | len)` i64.
//!
//! ## TODO
//! - [ ] Implement `PluginStore` with per-plugin `wasmtime::Module` caching
//! - [ ] Implement `host_log` import
//! - [ ] Implement `host_kv_get` / `host_kv_set` imports backed by a shared cache
//! - [ ] Add per-plugin fuel limits for CPU budget enforcement
//! - [ ] Add epoch-based interruption for wall-clock timeouts

use anyhow::{Context, Result};
use std::path::Path;
use wasmtime::{Engine, Linker, Module, Store};

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// A compiled, ready-to-instantiate WASM plugin.
pub struct Plugin {
    module: Module,
    engine: Engine,
}

impl Plugin {
    /// Compile a `.wasm` file.  Compilation is expensive; cache the result.
    ///
    /// # TODO
    /// - [ ] Accept `&[u8]` in addition to a path
    /// - [ ] Validate the module exports `on_request`
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let engine = Engine::default();
        let module = Module::from_file(&engine, path.as_ref())
            .with_context(|| format!("compiling {:?}", path.as_ref()))?;
        Ok(Self { module, engine })
    }
}

/// A single invocation context.  Create one per request.
pub struct PluginCall<'p> {
    plugin: &'p Plugin,
}

impl<'p> PluginCall<'p> {
    pub fn new(plugin: &'p Plugin) -> Self {
        Self { plugin }
    }

    /// Run `on_request` with the serialised request bytes.
    ///
    /// Returns the raw action bytes written by the plugin.
    ///
    /// # TODO
    /// - [ ] Write `request_bytes` into plugin linear memory
    /// - [ ] Link host imports (host_log, host_kv_get, …)
    /// - [ ] Call `on_request(ptr, len)` and read back the action
    pub fn on_request(&self, _request_bytes: &[u8]) -> Result<Vec<u8>> {
        let mut store: Store<()> = Store::new(&self.plugin.engine, ());
        let linker: Linker<()> = Linker::new(&self.plugin.engine);

        // TODO: linker.func_wrap("env", "host_log", |…| { … })?;

        let _instance = linker
            .instantiate(&mut store, &self.plugin.module)
            .context("instantiating plugin")?;

        // TODO: get `on_request` typed func, write memory, call, read result
        unimplemented!("on_request execution")
    }
}

// ---------------------------------------------------------------------------
// Plugin registry
// ---------------------------------------------------------------------------

/// Caches compiled plugins by file path.
///
/// # TODO
/// - [ ] Implement LRU eviction
/// - [ ] Watch plugin files for changes and recompile
pub struct PluginRegistry {
    // TODO: HashMap<PathBuf, Arc<Plugin>>
}

impl PluginRegistry {
    pub fn new() -> Self { Self {} }

    /// Load (or return cached) plugin for `path`.
    pub fn get_or_load(&mut self, _path: &Path) -> Result<Plugin> {
        // TODO: check cache, compile if missing
        unimplemented!("plugin registry")
    }
}

impl Default for PluginRegistry {
    fn default() -> Self { Self::new() }
}
