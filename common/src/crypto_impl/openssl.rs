use crate::{Result, error::Error};
use openssl::{
    asn1::Asn1Time,
    hash::MessageDigest,
    pkey::{PKey, Private},
    ssl::{Ssl, SslContext, SslContextBuilder, SslMethod, SslStream, SslVerifyMode},
    x509::X509Builder,
};
use std::io::{Read, Write};

const CIPHER: &str = "TLS_AES_128_GCM_SHA256";

pub type CryptoStream<S> = SslStream<S>;

pub fn ssl_start_client<T>(stream: T) -> Result<CryptoStream<T>>
where
    T: Read + Write,
{
    let mut s = ssl_start(stream, SslMethod::tls_client(), no_extras);
    s.connect()?;
    Ok(s)
}

pub fn ssl_start_server<T>(stream: T) -> Result<CryptoStream<T>>
where
    T: Read + Write,
{
    let mut s = ssl_start(stream, SslMethod::tls_server(), add_certificate);
    s.accept()?;
    Ok(s)
}

fn ssl_start<T>(
    stream: T,
    method: SslMethod,
    extra_builder: impl FnOnce(SslContextBuilder, &PKey<Private>) -> SslContextBuilder,
) -> CryptoStream<T>
where
    T: Read + Write,
{
    let pkey = PKey::generate_ed25519().expect("Failed to generate new private key");

    let mut b = SslContext::builder(method).expect("Failed to create SslContext Builder");
    b.set_ciphersuites(CIPHER).expect("Failed to set ciphers");
    b.set_private_key(&pkey).expect("Failed to set private key");

    // we are primarily interested in encryption, not authentication
    b.set_verify(SslVerifyMode::NONE);

    b = extra_builder(b, &pkey);

    let ctx = b.build();
    let ssl = Ssl::new(&ctx).expect("Failed to create Ssl from Context");

    SslStream::new(ssl, stream).expect("Failed to create SslStream")
}

fn no_extras(b: SslContextBuilder, _: &PKey<Private>) -> SslContextBuilder {
    b
}

fn add_certificate(mut b: SslContextBuilder, pkey: &PKey<Private>) -> SslContextBuilder {
    let asn1_time = Asn1Time::days_from_now(0).expect("Failed to create Asn1Time");
    let mut x509_b = X509Builder::new().expect("Failed to construct x509 builder");

    x509_b
        .set_pubkey(pkey)
        .expect("Failed to set pubkey for x509");
    x509_b
        // uses PureEdDSA -> no digest: https://docs.openssl.org/3.0/man7/EVP_SIGNATURE-ED25519/#ed25519-and-ed448-signature-parameters
        .sign(pkey, MessageDigest::null())
        .expect("Failed to sign x509 with privkey");
    x509_b
        .set_not_before(&asn1_time)
        .expect("Failed to set not_before");
    x509_b
        .set_not_after(&asn1_time)
        .expect("Failed to set not_after");

    let cert = x509_b.build();

    b.set_certificate(&cert)
        .expect("Failed to set certificate for connection");
    b
}

impl From<openssl::ssl::Error> for Error {
    fn from(e: openssl::ssl::Error) -> Self {
        Error::MiscError(e.to_string())
    }
}
