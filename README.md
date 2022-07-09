# Vanity Arweave Wallet Generator

> _W.A.V.E_, if you squint your eyes hard enough

This is just a "Hello World" project I made to learn Rust. (See [Go version](https://github.com/maximousblk/wave))

## Usage

```
λ wave -h
wave 0.1.0
Maximous Black <maximousblk@gmail.com>
Vanity Arweave Wallet Generator

USAGE:
    wave [OPTIONS] <PATTERN>

ARGS:
    <PATTERN>    Pattern to use for the wallet names

OPTIONS:
    -h, --help                 Print help information
    -o, --output <OUTPUT>      The path to the output directory [default: ./wallets]
    -t, --threads <THREADS>    The number of threads to use [default: 4]
    -V, --version              Print version information
```

### Example

```
λ wave '^[^-_]+$'

🔍 Pattern: /^[^-_]+$/
📁 Output directory: ./wallets
🧵 Threads: 4

🔍 [T02] wallet: vwLmxaLBLCZm_FumoaqmNAcJVXyEyua9WXCRA2oqwXE
🔍 [T01] wallet: jEJ4tVurPrJHOdAC8kLi5Sm63YRamkIZ9VV-NtF5h1Q
🔍 [T00] wallet: tSzyY2UZNy8EKhPt-b_GCDxRmv7_0Vn-9Vfe1BIwqCY
🔍 [T02] wallet: ozG_UVdta9O_dIr8p1X5CQEvZePntnI4-kNkHrjnV2A
🔍 [T03] wallet: s3ryItkxfZd2jMm5mkXqZC49vE_HxUcsYSG-LV6m1OU
🔍 [T01] wallet: sbCOmzv9X0q6IuET6j4zpo_xni5VAj_97aTrfM2MiQo
🔍 [T02] wallet: 5DH-od2FRLhOq-4JCblcMmZol-bKQDjmyW4PXNuBSlk
🔍 [T00] wallet: i9-FNmfn5ZWefM9fUn4M6FXaDFsGSEKCUG5kbxvHowU
🔍 [T03] wallet: gc18ziVQESh0d8QoXYa3wG_q5KrNoNtJEhXmQnh6etc
🔍 [T01] wallet: TDZWJbDx8Xn4UZeFMdgBG3xa_XIlcvKU3Gby8x1t6Eg

✅ [T02] wallet: oGoO2xuAHXeZChqMrVjzfFdpRHfCTIr7FNDNhsln4lw

📄 wallet written to file ./wallets/arweave-keyfile-oGoO2xuAHXeZChqMrVjzfFdpRHfCTIr7FNDNhsln4lw.json
🏁 Done in 15.290s (0.7/s)
```

## License

This software is licensed under [The MIT License](./LICENSE).
