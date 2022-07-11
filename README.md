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
🧵 Threads: 8

🔍 [T05] wallet: yEGjF6DJphza_5qmt3jmtION_x22Us8oliHaxgc5Bkg
🔍 [T02] wallet: yUMqXwQbm7yG8GA6lU5I4Ofc3KWb_YBJM2hPxW_yXeI
🔍 [T07] wallet: nGmPci4H1pPvmsSZcpPKYRId3HAxCc_ZXbVzkmKQ44I
🔍 [T04] wallet: _WDslHAWuK30GD7WG0RtdIFxskromKOcDl2-5WSIAiE
🔍 [T03] wallet: 6oQXEJofqwBHKALnd1fN3FzTozkNN_SvreTmsgdX8qA
🔍 [T07] wallet: p8EOw1eltIeR7aruNN-RdC63Sk7I87f813bXNjXOKRg
🔍 [T06] wallet: U_jr611hJpd81P7Fax9Z9wW-bA6no501K5XNeoOzt3Y
🔍 [T00] wallet: Rx73SLoAqq0zkZ-fpPj84I5M6FAq_rX2mXUDjTm7Oic
🔍 [T01] wallet: 7Hm_r0-sWyoJ_TSyr48jRag6MqxwBCyhSF3XrWe3h1w
🔍 [T05] wallet: ok8sX8SLY3ALkoSsX0DHo69r9X9gsVJAjUTutp7Y-gg
🔍 [T03] wallet: 95F5P3sAv1QcfMk60-x3_yNSxGuGInW_mH3W0WBZKOI
🔍 [T02] wallet: sBBUYmkZ3zj5twljNLCDTgoZJA747GjLfhzpW-si8ig
🔍 [T04] wallet: 2L1TQV5rcN7czKO-rPUhhDhktFrS0IDiFrLHuk766WQ

✅ [T01] wallet: TreSe3qCrs1n0lKf6jfCYtWAOD1koJqUHeXsDWA60zU
📄 wallet written to file ./wallets/arweave-keyfile-TreSe3qCrs1n0lKf6jfCYtWAOD1koJqUHeXsDWA60zU.json

🏁 Done in 10.512s (1.3/s)
```

## License

This software is licensed under [The MIT License](./LICENSE).
