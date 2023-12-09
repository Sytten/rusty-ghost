use std::io::BufReader;

use hudsucker::certificate_authority::{CertificateAuthority, RcgenAuthority};
use rustls_pemfile as pemfile;
use tokio::fs;

use crate::config::GhostConfig;

pub async fn load(config: &GhostConfig) -> impl CertificateAuthority {
    // Private key
    let private_key_bytes = fs::read(&config.private_key)
        .await
        .expect("Failed to load private key");
    let mut reader = BufReader::new(&private_key_bytes[..]);
    let private_key = rustls::PrivateKey(
        pemfile::pkcs8_private_keys(&mut reader)
            .next()
            .unwrap()
            .expect("Failed to parse private key")
            .secret_pkcs8_der()
            .to_vec(),
    );

    let ca_cert_bytes = fs::read(&config.ca_cert)
        .await
        .expect("Failed to load CA certificate");
    let mut reader = BufReader::new(&ca_cert_bytes[..]);
    let ca_cert = rustls::Certificate(
        pemfile::certs(&mut reader)
            .next()
            .unwrap()
            .expect("Failed to parse CA certificate")
            .to_vec(),
    );

    RcgenAuthority::new(private_key, ca_cert, 1_000)
        .expect("Failed to create Certificate Authority")
}
