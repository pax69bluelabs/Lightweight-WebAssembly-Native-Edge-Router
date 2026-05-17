//! Minimal example WASM plugin. Compile with:
//!   cargo build -p hello-plugin --target wasm32-unknown-unknown

// No-op on_request: just forwards every request unchanged.
// TODO: use plugin_sdk types once read_request / encode_action are implemented.
#[no_mangle]
pub extern "C" fn on_request(_ptr: u32, _len: u32) -> u64 {
    0
}
