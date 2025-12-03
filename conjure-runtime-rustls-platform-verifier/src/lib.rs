/// A partial fork of `conjure-runtime` that exposes types required to implement custom raw client builders.
pub mod conjure_runtime_raw {
    pub use ::conjure_runtime_raw::*;
}

pub use conjure_runtime;
pub use rustls_platform_verifier;

mod raw_client_builder;

pub use raw_client_builder::*;

pub type PlatformVerifierClient =
    conjure_runtime::Client<conjure_runtime_raw::raw::DefaultRawClient>;

pub type ResponseBody = conjure_runtime::ResponseBody<conjure_runtime_raw::raw::DefaultRawBody>;

pub mod crypto;
