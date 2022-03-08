# idis_rs

iBoot disassembler toy made to learn a Rust, it's mostly based on [idis](https://github.com/matteyeux/idis).

#### Build 

To build `idis_rs` run `cargo build`

#### Usage
```
idis_rs 0.1.0

USAGE:
    idis_rs [OPTIONS] --file <FILE>

OPTIONS:
    -c, --count <COUNT>    [default: 0]
    -f, --file <FILE>......
    -h, --help             Print help information
    -s, --skip <SKIP>      [default: 0]
    -V, --version          Print version information
```

#### Run

To run with `cargo` :
```
λ ~/dev/idis_rs(clap*) » cargo run -- -f iBoot.n841.RELEASE.15.4b3.bin -c 10
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/idis_rs -f iBoot.n841.RELEASE.15.4b3.bin -c 10`
iBoot version : 7459.100.504.0.1
Base address 0x19c034000
19c034000: adrp x0, 0x19c034000
19c034004: add x0, x0, #0x0
19c034008: ldr x1, 0x19c034300
19c03400c: bl 0x19c065ec0
19c034010: cmp x1, x0
19c034014: b.eq 0x19c034154
19c034018: ldr x2, 0x19c034308
19c03401c: sub x2, x2, x1
19c034020: add x2, x2, #0x3f
19c034024: and x2, x2, #0xffffffffffffffc0
```

#### Credits

- yrp for [bad64](https://github.com/yrp604/bad64) (the actual disassembler)
