use base64_url::encode;
use rsa::{BigUint, PublicKeyParts, RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::ops::Rem;

pub struct Wallet {
    pub pvtkey: RsaPrivateKey,
    pub pubkey: RsaPublicKey,
}

impl Wallet {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        let pvtkey = RsaPrivateKey::new_with_exp(
            &mut rng,
            4096,
            &BigUint::from_bytes_be(&[0x01, 0x00, 0x01]),
        )
        .unwrap();
        let pubkey = RsaPublicKey::from(&pvtkey);

        Wallet { pvtkey, pubkey }
    }

    pub fn address(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.components().n.to_bytes_be());
        encode(&hasher.finalize()[..])
    }

    pub fn components(&self) -> Components {
        let d = self.pvtkey.d();
        let p = &self.pvtkey.primes()[0];
        let q = &self.pvtkey.primes()[1];

        Components {
            e: self.pubkey.e().clone(),
            d: d.clone(),

            p: p.clone(),
            q: q.clone(),
            n: self.pubkey.n().clone(),

            dp: d.rem(p - BigUint::from(1u8)),
            dq: d.rem(q - BigUint::from(1u8)),
            qi: p.modpow(&(q - BigUint::from(2u8)), &q),
        }
    }

    pub fn jwk(&self) -> JWK {
        JWK {
            kty: "RSA".to_string(),
            ext: true,

            d: encode(&self.components().d.to_bytes_be()),
            e: encode(&self.components().e.to_bytes_be()),

            p: encode(&self.components().p.to_bytes_be()),
            q: encode(&self.components().q.to_bytes_be()),
            n: encode(&self.components().n.to_bytes_be()),

            dp: encode(&self.components().dp.to_bytes_be()),
            dq: encode(&self.components().dq.to_bytes_be()),
            qi: encode(&self.components().qi.to_bytes_be()),
        }
    }
}

pub struct Components {
    e: BigUint,
    d: BigUint,

    p: BigUint,
    q: BigUint,
    n: BigUint,

    dp: BigUint,
    dq: BigUint,
    qi: BigUint,
}

#[derive(Serialize, Deserialize)]
pub struct JWK {
    kty: String,
    ext: bool,

    e: String,
    d: String,

    p: String,
    q: String,
    n: String,

    dp: String,
    dq: String,
    qi: String,
}
