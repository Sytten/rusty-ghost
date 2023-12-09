use std::sync::Arc;

use hyper::client::HttpConnector;
use hyper::Client;
use hyper_rustls::{ConfigBuilderExt, HttpsConnector, HttpsConnectorBuilder};
use rustls::ClientConfig;

/// We need a custom client since some torrent clients
/// send the IP instead of the domain of the tracker for
/// some fucked up reason, so the TLS cert is invalid.
/// So we tell the client to ignore those errors.
pub fn build() -> Client<HttpsConnector<HttpConnector>> {
    let mut config = ClientConfig::builder()
        .with_safe_defaults()
        .with_webpki_roots()
        .with_no_client_auth();

    config
        .dangerous()
        .set_certificate_verifier(Arc::new(danger::NoCertificateVerification));

    let https = HttpsConnectorBuilder::new()
        .with_tls_config(config)
        .https_or_http()
        .enable_http1();

    let https = https.build();

    Client::builder()
        .http1_title_case_headers(true)
        .http1_preserve_header_case(true)
        .build(https)
}

mod danger {
    use std::time::SystemTime;

    use rustls::{Certificate, ServerName};

    #[derive(Debug)]
    pub struct NoCertificateVerification;

    impl rustls::client::ServerCertVerifier for NoCertificateVerification {
        fn verify_server_cert(
            &self,
            _end_entity: &Certificate,
            _intermediates: &[Certificate],
            _server_name: &ServerName,
            _scts: &mut dyn Iterator<Item = &[u8]>,
            _ocsp_response: &[u8],
            _now: SystemTime,
        ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
            Ok(rustls::client::ServerCertVerified::assertion())
        }
    }
}
