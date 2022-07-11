use clap::Parser;
use colored::*;
use path_slash::PathBufExt as _;
use regex::Regex;
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;
use update_informer::{registry, Check};
use wave::Wallet;

#[derive(Parser)]
#[clap(author, version = {option_env!("RELEASE_VERSION").unwrap_or("dev")}, about, arg_required_else_help(true))]
struct Args {
  /// The number of threads to use
  #[clap(short, long, value_parser, default_value_t = 8)]
  threads: u8,

  /// The path to the output directory
  #[clap(short, long, value_parser, default_value("./wallets"))]
  output: String,

  /// Pattern to use for the wallet names
  #[clap(value_parser)]
  pattern: String,
}

fn main() {
  let current_version = option_env!("RELEASE_VERSION").unwrap_or("dev");

  let informer = update_informer::new(registry::GitHub, "maximousblk/wave.rs", current_version);

  if let Some(version) = informer.check_version().ok().flatten() {
    let msg = format!("A new release of {} is available: v{} -> {}", "wave".italic().cyan(), current_version, version.to_string().green());

    let release_url = format!("https://github.com/maximousblk/wave.rs/releases/tag/{}", version).yellow();

    println!("\n{msg}\n{url}", msg = msg, url = release_url);
  }

  let args = Args::parse();

  let pattern = Regex::new(&args.pattern).expect("Invalid pattern");

  let outdir = PathBuf::from(&args.output);

  if !outdir.exists() || !outdir.is_dir() {
    fs::create_dir_all(&outdir).expect("Failed to create output directory");
  }

  println!("\n🔍 Pattern: {}", format!("/{}/", &args.pattern).yellow());
  println!("📁 Output directory: {}", &outdir.to_slash_lossy().to_string().yellow());
  println!("🧵 Threads: {}\n", &args.threads.to_string().yellow());

  let (tx, rx) = mpsc::channel();

  let now = Instant::now();

  for t in 0..args.threads {
    let tx = tx.clone();
    thread::spawn(move || loop {
      tx.send((t, Wallet::new())).expect("Failed to send jwt");
    });
  }

  let mut count: u32 = 0;
  for (t, wallet) in rx {
    count += 1;
    let address = &wallet.address();
    if pattern.is_match(address) {
      println!("\n✅ {} wallet: {}", format!("[T{:02}]", t).dimmed(), address.green().bold());

      // write wallet jwk to file
      let outfile = outdir.join(format!("arweave-keyfile-{}.json", address));

      let jwk_json = serde_json::to_string(&wallet.jwk()).expect("Failed to serialize jwk");

      fs::write(&outfile, jwk_json).expect("Failed to write wallet to file");

      println!("📄 wallet written to file {}\n", outfile.to_slash_lossy().to_string().yellow());

      let el = now.elapsed().as_secs_f32();
      println!("🏁 Done in {:.3}s {}", el.to_string().bold(), format!("({:.1}/s)", (count as f32 / el)).dimmed());

      break;
    } else {
      println!("🔍 {} wallet: {}", format!("[T{:02}]", t).dimmed(), address);
    }
  }
}
