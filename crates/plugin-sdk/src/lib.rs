//! # plugin-sdk
//!
//! Types and host-function declarations shared between the router host and
//! WASM plugin authors.  Compile your plugin to `wasm32-unknown-unknown` and
//! depend on this crate.
//!
//! ## Minimal plugin
//! ```rust,no_run
//! use plugin_sdk::{Request, Response, Action};
//!
//! #[no_mangle]
//! pub extern "C" fn on_request(ptr: u32, len: u32) -> u64 {
//!     let req = plugin_sdk::read_request(ptr, len);
//!     // inspect / mutate req …
//!     plugin_sdk::encode_action(Action::Forward(req))
//! }
//! ```

#![cfg_attr(target_arch = "wasm32", no_std)]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

// ---------------------------------------------------------------------------
// Wire types (JSON-serialisable, passed through linear WASM memory)
// ---------------------------------------------------------------------------

/// Subset of an HTTP request passed into a plugin.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Request {
    pub method: String,
    pub uri: String,
    pub headers: Vec<(String, String)>,
    /// Body bytes (may be empty for GET/HEAD)
    pub body: Vec<u8>,
}

/// What the plugin wants the router to do next.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Action {
    /// Pass the (possibly mutated) request to the upstream.
    Forward(Request),
    /// Short-circuit with a synthetic response.
    Respond(Response),
    /// Abort with an error status.
    Deny { status: u16, message: String },
}

/// Synthetic HTTP response a plugin can return directly.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Response {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

// ---------------------------------------------------------------------------
// Host-function imports (only compiled when targeting wasm32)
// ---------------------------------------------------------------------------

#[cfg(target_arch = "wasm32")]
extern "C" {
    /// Write `len` bytes starting at `ptr` to the host log.
    pub fn host_log(ptr: u32, len: u32);

    // TODO: add host_kv_get / host_kv_set for shared cache access
    // TODO: add host_stellar_verify for on-chain auth checks
}

// ---------------------------------------------------------------------------
// Helpers for reading/writing through WASM linear memory
// ---------------------------------------------------------------------------

/// Decode a `Request` from a pointer+length pair written by the host.
///
/// # Safety
/// The host guarantees `ptr..ptr+len` is valid UTF-8 JSON.
///
/// # TODO
/// - [ ] Switch from JSON to a zero-copy binary format (e.g. flatbuffers)
#[cfg(target_arch = "wasm32")]
pub fn read_request(ptr: u32, len: u32) -> Request {
    // SAFETY: host wrote valid JSON into our linear memory
    let slice = unsafe { core::slice::from_raw_parts(ptr as *const u8, len as usize) };
    // TODO: replace with a proper deserialiser; serde_json pulls in std
    let _ = slice;
    unimplemented!("deserialise Request from JSON bytes")
}

/// Encode an `Action` and return a packed `(ptr << 32 | len)` i64.
///
/// # TODO
/// - [ ] Allocate in WASM linear memory and return pointer to host
#[cfg(target_arch = "wasm32")]
pub fn encode_action(_action: Action) -> u64 {
    unimplemented!("serialise Action into WASM linear memory")
}
