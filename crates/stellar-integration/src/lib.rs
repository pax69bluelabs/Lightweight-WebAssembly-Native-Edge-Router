//! # stellar-integration
//!
//! Thin async client for the Stellar network used by router plugins and the
//! router core for on-chain operations such as:
//!
//! - Verifying a bearer token against a Soroban auth contract
//! - Reading per-account rate-limit state from a Soroban contract
//! - Emitting routing-event transactions to the Stellar ledger
//!
//! ## Stellar primer
//! Stellar is a fast, low-fee L1 blockchain.  **Soroban** is its WASM-based
//! smart-contract platform (contracts are Rust → wasm32).  **Horizon** is the
//! REST API gateway to the network.
//!
//! ## TODO
//! - [ ] Implement `HorizonClient` with reqwest/hyper
//! - [ ] Implement `SorobanClient` wrapping the JSON-RPC endpoint
//! - [ ] Add `AuthVerifier` that calls a Soroban contract to validate JWTs
//! - [ ] Add `RateLimitLedger` that reads/writes per-key counters on-chain

use anyhow::Result;

// ---------------------------------------------------------------------------
// Network config
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub horizon_url: String,
    pub network_passphrase: String,
    pub soroban_rpc_url: Option<String>,
}

impl NetworkConfig {
    pub fn testnet() -> Self {
        Self {
            horizon_url: "https://horizon-testnet.stellar.org".into(),
            network_passphrase: "Test SDF Network ; September 2015".into(),
            soroban_rpc_url: Some("https://soroban-testnet.stellar.org".into()),
        }
    }

    pub fn mainnet() -> Self {
        Self {
            horizon_url: "https://horizon.stellar.org".into(),
            network_passphrase: "Public Global Stellar Network ; September 2015".into(),
            soroban_rpc_url: Some("https://soroban-mainnet.stellar.org".into()),
        }
    }
}

// ---------------------------------------------------------------------------
// Horizon REST client
// ---------------------------------------------------------------------------

/// Async client for the Horizon REST API.
///
/// # TODO
/// - [ ] Add `get_account(account_id)` → `AccountResponse`
/// - [ ] Add `submit_transaction(xdr)` → `TransactionResult`
/// - [ ] Add connection pooling and retry with exponential back-off
#[allow(dead_code)]
pub struct HorizonClient {
    base_url: String,
    // TODO: http_client: hyper::Client<…>
}

impl HorizonClient {
    pub fn new(config: &NetworkConfig) -> Self {
        Self {
            base_url: config.horizon_url.clone(),
        }
    }

    /// Fetch account details from Horizon.
    ///
    /// # TODO
    /// - [ ] Implement HTTP GET `{base_url}/accounts/{account_id}`
    pub async fn get_account(&self, _account_id: &str) -> Result<serde_json::Value> {
        unimplemented!("GET /accounts/:id")
    }
}

// ---------------------------------------------------------------------------
// Soroban JSON-RPC client
// ---------------------------------------------------------------------------

/// Client for the Soroban smart-contract RPC endpoint.
///
/// # TODO
/// - [ ] Implement `simulate_transaction` for read-only contract calls
/// - [ ] Implement `send_transaction` for state-mutating calls
/// - [ ] Add XDR encoding helpers (use `stellar-xdr` crate)
#[allow(dead_code)]
pub struct SorobanClient {
    rpc_url: String,
}

impl SorobanClient {
    pub fn new(config: &NetworkConfig) -> Option<Self> {
        config.soroban_rpc_url.as_ref().map(|url| Self {
            rpc_url: url.clone(),
        })
    }

    /// Call a read-only Soroban contract function.
    ///
    /// # TODO
    /// - [ ] Build `simulateTransaction` JSON-RPC request
    /// - [ ] Decode XDR response into `serde_json::Value`
    pub async fn call_readonly(
        &self,
        _contract_id: &str,
        _function: &str,
        _args: &[serde_json::Value],
    ) -> Result<serde_json::Value> {
        unimplemented!("soroban simulateTransaction")
    }
}

// ---------------------------------------------------------------------------
// Auth verifier (uses a Soroban contract)
// ---------------------------------------------------------------------------

/// Verifies a bearer token by calling a Soroban auth contract.
///
/// # TODO
/// - [ ] Define the expected Soroban contract interface (SEP-10 / custom)
/// - [ ] Cache verification results with a short TTL
#[allow(dead_code)]
pub struct AuthVerifier {
    soroban: SorobanClient,
    contract_id: String,
}

impl AuthVerifier {
    pub fn new(soroban: SorobanClient, contract_id: impl Into<String>) -> Self {
        Self {
            soroban,
            contract_id: contract_id.into(),
        }
    }

    /// Returns `true` if `token` is valid according to the on-chain contract.
    pub async fn verify(&self, _token: &str) -> Result<bool> {
        // TODO: call self.soroban.call_readonly(&self.contract_id, "verify", &[token])
        unimplemented!("on-chain token verification")
    }
}
