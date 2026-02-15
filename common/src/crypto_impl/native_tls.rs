use crate::{Error, Result};
use native_tls::{HandshakeError, TlsConnector, TlsStream};
use std::{
    fmt::Debug,
    io::{Read, Write},
};

pub type CryptoStream<S> = TlsStream<S>;

pub fn ssl_start_server<T>(_: T) -> Result<CryptoStream<T>>
where
    T: Read + Write,
{
    unimplemented!()
}

pub fn ssl_start_client<T>(stream: T) -> Result<CryptoStream<T>>
where
    T: Read + Write + Debug,
{
    let connector = TlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .danger_accept_invalid_hostnames(true)
        .disable_built_in_roots(true)
        .use_sni(false)
        .min_protocol_version(Some(native_tls::Protocol::Tlsv12))
        .build()
        .expect("Failed to construct TlsConnector");

    eprintln!("using native-tls");

    connector.connect("", stream).map_err(|e| e.into())
}

impl<S> From<HandshakeError<S>> for Error {
    fn from(_: HandshakeError<S>) -> Self {
        Error::OtherError("HandshakeError")
    }
}
