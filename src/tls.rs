use std::fs::{File};
use std::env;
use std::sync::Arc;
use tokio_rustls::rustls::{Certificate, PrivateKey, ServerConfig};
use tokio_rustls::TlsAcceptor;
use tls_listener::TlsListener;
use hyper::server::conn::AddrIncoming;
use rustls_pemfile::{read_one, certs, Item};
use std::io::BufReader;

pub fn get_tls_listener() -> TlsListener<AddrIncoming, TlsAcceptor> {
    let addr = ([0, 0, 0, 0], 3000).into();

    let cert_path = env::var("CERT_PATH").unwrap();
    let mut cert_file_reader = BufReader::new(File::open(cert_path).unwrap());
    let certs = certs(&mut cert_file_reader).unwrap()
    .iter()
    .map(|v| Certificate(v.clone()))
    .collect();

    let key_path = env::var("KEY_PATH").unwrap();
    let mut key_file = BufReader::new(File::open(key_path).unwrap());
    let key = match read_one(&mut key_file).unwrap() {
        Some(Item::PKCS8Key(key)) => PrivateKey(key),
        // TODO: Support other formats?
        Some(_) => panic!("Unsopported private key format"),
        None => panic!("Empty privatekey file"),
    };

    let config = ServerConfig::builder()
    .with_safe_defaults()
    .with_no_client_auth()
    .with_single_cert(certs, key).unwrap();

    let acceptor = Arc::new(config).into();

    TlsListener::new(acceptor, AddrIncoming::bind(&addr).unwrap())
}