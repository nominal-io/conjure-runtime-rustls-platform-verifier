use std::sync::{Arc, OnceLock};

pub use rustls;

use rustls::crypto::CryptoProvider;

/// Returns the shared crypto provider used by this crate. This uses [`rustls::crypto::ring::default_provider()`];
pub fn ring_crypto_provider<'a>() -> &'a Arc<CryptoProvider> {
    static PROVIDER: OnceLock<Arc<CryptoProvider>> = OnceLock::new();

    PROVIDER.get_or_init(|| Arc::new(rustls::crypto::ring::default_provider()))
}
