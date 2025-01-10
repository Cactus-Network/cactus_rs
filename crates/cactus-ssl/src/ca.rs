use lazy_static::lazy_static;
use rcgen::{Certificate, CertificateParams, KeyPair};

pub const CACTUS_CA_KEY: &str = include_str!("../cactus_ca.key");
pub const CACTUS_CA_CRT: &str = include_str!("../cactus_ca.crt");

lazy_static! {
    pub static ref CACTUS_CA: Certificate = load_ca_cert();
    pub static ref CACTUS_CA_KEY_PAIR: KeyPair =
        KeyPair::from_pem(CACTUS_CA_KEY).expect("could not load CA keypair");
}

fn load_ca_cert() -> Certificate {
    let params =
        CertificateParams::from_ca_cert_pem(CACTUS_CA_CRT).expect("could not create CA params");
    params
        .self_signed(&CACTUS_CA_KEY_PAIR)
        .expect("could not create certificate")
}
