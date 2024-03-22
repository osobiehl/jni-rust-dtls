use coap::client::CoAPClient;
use coap::dtls::{DtlsConfig, DtlsConnection, DtlsResponse};
use coap::UdpCoAPClient;

use coap_lite::{CoapOption, CoapRequest, RequestType as Method};
use log::debug;
use pkcs8::{LineEnding, SecretDocument};
use rcgen::{KeyPair, PKCS_ECDSA_P256_SHA256, PKCS_ED25519};
use ring::signature::Ed25519KeyPair;
use std::fs::File;
use std::io::{BufReader, Cursor, Read};
use std::net::{SocketAddr, ToSocketAddrs};
use std::time::Duration;
use std::time::Instant;
use tokio::net::UdpSocket;
use tokio::sync::mpsc;
use webrtc_dtls::cipher_suite::{CipherSuite, CipherSuiteId};
use webrtc_dtls::config::{ClientAuthType, Config, ExtendedMasterSecretType};
use webrtc_dtls::crypto::{Certificate, CryptoPrivateKey, CryptoPrivateKeyKind, OID_ED25519};
use webrtc_dtls::listener::listen;

const SERVER_CERTIFICATE_PRIVATE_KEY: &'static str = include_str!("../crts/server/srv-key.pem");
const SERVER_CERTIFICATE: &'static str = include_str!("../crts/server/srv-cert.pem");
const ROOT_CERTIFICATE: &'static str = include_str!("../crts/root_ca.pem");
const CLIENT_CERTIFICATE_PRIVATE_KEY: &'static str = include_str!("../crts/client/client-key.pem");
const CLIENT_CERTIFICATE: &'static str = include_str!("../crts/client/client-crt.pem");
pub fn get_certificate(name: &str) -> rustls::Certificate {
    //    let Ok(mut f) = File::open(name) else {
    //        debug!("could not open file: {name}");
    //        panic!("could not");
    //    };
    //    let mut reader = BufReader::new(&mut f);
    let bytes = name.as_bytes();
    // Create a BufReader from the byte slice using Cursor
    let mut reader = BufReader::new(Cursor::new(bytes));
    let mut cert_iter = rustls_pemfile::certs(&mut reader);
    debug!("cert iter");
    let cert = cert_iter
        .next()
        .unwrap()
        .expect("could not get certificate");
    assert!(
        cert_iter.next().is_none(),
        "there should only be 1 certificate in this file"
    );
    debug!("got cert");
    return rustls::Certificate(cert.to_vec());
}
pub fn get_private_key(name: &str) -> CryptoPrivateKey {
    let mut reader = BufReader::new(Cursor::new(name.as_bytes()));
    let mut buf = vec![];
    reader.read_to_end(&mut buf).unwrap();
    let s = String::from_utf8(buf).expect("utf8 of file");
    // convert key to pkcs8
    //let s = convert_to_pkcs8(&s);

    debug!("getting  key!");
    let mut key_pair = KeyPair::from_pem(s.as_str()).expect("could not parse key");

    //
    //let ed_pair = Ed25519KeyPair::from_pkcs8_maybe_unchecked(s.as_str()).expect("key pair in file");
    CryptoPrivateKey::from_key_pair(&key_pair)
        .map(|k| {
            debug!("got key!");
            k
        })
        .expect("could not create key pair")
}

pub fn client_key() -> CryptoPrivateKey {
    return get_private_key(CLIENT_CERTIFICATE_PRIVATE_KEY);
}

pub fn server_certificate() -> rustls::Certificate {
    return get_certificate(SERVER_CERTIFICATE);
}
pub fn root_certificate() -> rustls::Certificate {
    return get_certificate(ROOT_CERTIFICATE);
}
pub fn client_certificate() -> rustls::Certificate {
    return get_certificate(CLIENT_CERTIFICATE);
}

pub async fn get_client() -> CoAPClient<DtlsConnection> {
    let server_port = 7777;
    let client_cfg = {
        let mut client_cert_pool = rustls::RootCertStore::empty();
        client_cert_pool
            .add(&root_certificate())
            .expect("ROOT CERTIFICATE!");
        let client_cert = client_certificate();
        let server_cert = server_certificate();
        client_cert_pool
            .add(&server_cert)
            .expect("could not add certificate");
        debug!("added client cert");

        let client_private_key = client_key();
        let certificate = Certificate {
            certificate: vec![client_cert],
            private_key: client_private_key,
        };

        Config {
            certificates: vec![certificate],
            roots_cas: client_cert_pool,
            mtu: 512,
            flight_interval: Duration::from_secs(60),
            server_name: "txng-draeger".to_string(),

            cipher_suites: vec![
                CipherSuiteId::Tls_Ecdhe_Ecdsa_With_Aes_128_Ccm,
                CipherSuiteId::Tls_Ecdhe_Ecdsa_With_Aes_128_Ccm_8,
                CipherSuiteId::Tls_Ecdhe_Ecdsa_With_Aes_128_Gcm_Sha256,
                CipherSuiteId::Tls_Ecdhe_Rsa_With_Aes_128_Gcm_Sha256,
                CipherSuiteId::Tls_Ecdhe_Ecdsa_With_Aes_256_Cbc_Sha,
                CipherSuiteId::Tls_Ecdhe_Rsa_With_Aes_256_Cbc_Sha,
                CipherSuiteId::Tls_Psk_With_Aes_128_Ccm,
                CipherSuiteId::Tls_Psk_With_Aes_128_Ccm_8,
                CipherSuiteId::Tls_Psk_With_Aes_128_Gcm_Sha256,
            ],
            ..Default::default()
        }
    };
    let dtls_config = DtlsConfig {
        config: client_cfg,
        dest_addr: ("10.0.2.2", server_port)
            .to_socket_addrs()
            .unwrap()
            .next()
            .unwrap(),
    };
    debug!("created dtls config");

    let mut client = CoAPClient::from_dtls_config(dtls_config)
        .await
        .expect("could not create client");
    debug!("client created");
    client.set_receive_timeout(Duration::from_secs(5));
    client.set_transport_retries(5);
    return client;
}

pub async fn client_get() {
    let start_time = Instant::now();
    let mut result = Vec::new();
    for c in b'a'..=b'z' {
        result.extend(std::iter::repeat(c).take(1024));
    }
    let resp = UdpCoAPClient::get("coap://10.0.2.2:7777/block")
        .await
        .unwrap();
    let end_time = Instant::now();
    let duration = end_time - start_time;

    debug!("response len: {:?}", resp.message.payload.len());
    debug!("duration: {} ms", duration.as_millis());
    debug!(
        "throughput: {}B/s",
        1000.0 * resp.message.payload.len() as f32 / duration.as_millis() as f32
    );
}
