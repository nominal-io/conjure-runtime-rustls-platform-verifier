use conjure_error::Error;
use conjure_runtime::raw::BuildRawClient;
use conjure_runtime::{Builder, builder};
use conjure_runtime_raw::raw::DefaultRawClient;
use conjure_runtime_raw::raw::{HTTP_KEEPALIVE, TCP_KEEPALIVE};
use conjure_runtime_raw::service::proxy::ProxyConfig;
use conjure_runtime_raw::service::proxy::connector::ProxyConnectorLayer;
use conjure_runtime_raw::service::timeout::TimeoutLayer;
use conjure_runtime_raw::service::tls_metrics::TlsMetricsLayer;
use hyper_rustls::HttpsConnectorBuilder;
use hyper_util::client::legacy::Client;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::rt::{TokioExecutor, TokioTimer};
use rustls::ClientConfig;
use rustls_platform_verifier::BuilderVerifierExt;
use tower_layer::Layer;

#[derive(Copy, Clone)]
pub struct RawClientBuilder;

// This code is copied and adapted from conjure-runtime's DefaultRawClientBuilder
impl BuildRawClient for RawClientBuilder {
    type RawClient = DefaultRawClient;

    fn build_raw_client(
        &self,
        builder: &Builder<builder::Complete<Self>>,
    ) -> Result<Self::RawClient, Error> {
        let mut connector = HttpConnector::new();
        connector.enforce_http(false);
        connector.set_nodelay(true);
        connector.set_keepalive(Some(TCP_KEEPALIVE));
        connector.set_connect_timeout(Some(builder.get_connect_timeout()));

        let proxy = ProxyConfig::from_config(builder.get_proxy())?;

        let connector = TimeoutLayer::new(builder).layer(connector);
        let connector = ProxyConnectorLayer::new(&proxy).layer(connector);

        let client_config =
            ClientConfig::builder_with_provider(crate::crypto::ring_crypto_provider().clone())
                .with_safe_default_protocol_versions()
                .map_err(Error::internal_safe)?
                .with_platform_verifier()
                .map_err(Error::internal_safe)?;

        let client_config = match (
            builder.get_security().cert_file(),
            builder.get_security().key_file(),
        ) {
            (Some(cert_file), Some(key_file)) => {
                let cert_chain = conjure_runtime_raw::raw::load_certs_file(cert_file)?;
                let private_key = conjure_runtime_raw::raw::load_private_key(key_file)?;

                client_config
                    .with_client_auth_cert(cert_chain, private_key)
                    .map_err(Error::internal_safe)?
            }
            (None, None) => client_config.with_no_client_auth(),
            _ => {
                return Err(Error::internal_safe(
                    "neither or both of key-file and cert-file must be set in the client \
                    security config",
                ));
            }
        };

        let connector = HttpsConnectorBuilder::new()
            .with_tls_config(client_config)
            .https_or_http()
            .enable_all_versions()
            .wrap_connector(connector);
        let connector = TlsMetricsLayer::new(builder).layer(connector);

        let client = Client::builder(TokioExecutor::new())
            .pool_idle_timeout(HTTP_KEEPALIVE)
            .pool_timer(TokioTimer::new())
            .timer(TokioTimer::new())
            .build(connector);

        Ok(DefaultRawClient(client))
    }
}
