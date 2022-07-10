use base64_url::encode;
use clap::Parser;
use regex::Regex;
use rsa::{BigUint, RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::ops::Rem;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

#[derive(Parser, Debug)]
#[clap(author, version, about, arg_required_else_help(true))]
struct Args {
    /// The number of threads to use
    #[clap(short, long, value_parser, default_value_t = 4)]
    threads: u8,

    /// The path to the output directory
    #[clap(short, long, value_parser, default_value("./wallets"))]
    output: String,

    /// Pattern to use for the wallet names
    #[clap(value_parser)]
    pattern: String,
}

fn main() {
    let args = Args::parse();

    let pattern = Regex::new(&args.pattern).expect("Invalid pattern");
    let outdir = PathBuf::from(&args.output);
    if !outdir.exists() {
        fs::create_dir_all(&outdir).expect("Failed to create output directory");
    }

    println!("\n🔍 Pattern: /{}/", &args.pattern);
    println!("📁 Output directory: {}", &outdir.to_str().unwrap());
    println!("🧵 Threads: {}\n", &args.threads);

    let (tx, rx) = mpsc::channel();

    let now = Instant::now();

    for t in 0..args.threads {
        let tx = tx.clone();
        thread::spawn(move || loop {
            tx.send((t, Wallet::new())).expect("Failed to send jwt");
        });
    }

    let mut count = 0;
    for (t, wallet) in rx {
        count += 1;
        if pattern.is_match(&wallet.address()) {
            println!("\n✅ [T{:02}] wallet: {}", t, wallet.address());

            // write wallet jwk to file
            let outfile = outdir.join(format!("arweave-keyfile-{}.json", wallet.address()));

            let jwk = wallet.jwk();
            let jwk_json = serde_json::to_string(&jwk).expect("Failed to serialize jwk");

            fs::write(&outfile, jwk_json).expect("Failed to write wallet to file");

            println!("📄 wallet written to file {}\n", outfile.display());

            break;
        } else {
            println!("🔍 [T{:02}] wallet: {}", t, wallet.address());
        }
    }

    let el = now.elapsed().as_secs_f32();
    println!("🏁 Done in {:.3}s ({:.1}/s)", el, (count as f32 / el));
}

#[derive(Serialize, Deserialize, Debug)]
struct JWK {
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

#[allow(dead_code)]
struct Components {
    e: Vec<u8>,
    d: BigUint,

    p: BigUint,
    q: BigUint,
    n: BigUint,

    dp: BigUint,
    dq: BigUint,
    qi: BigUint,
}

#[allow(dead_code)]
struct Wallet {
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
        let d = RsaPrivateKey::d(&self.pvtkey);
        let p = &RsaPrivateKey::primes(&self.pvtkey)[0];
        let q = &RsaPrivateKey::primes(&self.pvtkey)[1];

        Components {
            e: vec![0x01, 0x00, 0x01],
            d: d.clone(),

            p: p.clone(),
            q: q.clone(),
            n: p * q,

            dp: d.rem(p - BigUint::from(1u8)),
            dq: d.rem(q - BigUint::from(1u8)),
            qi: p.modpow(&(q - BigUint::from(2u8)), &q),
        }
    }

    pub fn jwk(&self) -> JWK {
        JWK {
            kty: "RSA".to_string(),
            ext: true,

            e: "AQAB".to_string(),
            d: encode(&self.components().d.to_bytes_be()),

            p: encode(&self.components().p.to_bytes_be()),
            q: encode(&self.components().q.to_bytes_be()),
            n: encode(&self.components().n.to_bytes_be()),

            dp: encode(&self.components().dp.to_bytes_be()),
            dq: encode(&self.components().dq.to_bytes_be()),
            qi: encode(&self.components().qi.to_bytes_be()),
        }
    }
}
