primes
======

Calulate prime numbers in `rust`, simple enough but also includes for reference code so that benchmarking (unstable in stable and beta releases of rust as of version 1.58.1) can be included in the source and but ignored when complied under stable using the crate `rustversion` and a build script to set features.

This is done by doing the following:

build.rs
```
#[rustversion::nightly]
fn main() {
    println!("cargo:rustc-cfg=test");
}

#[rustversion::not(nightly)]
fn main() { }
```

main.rs
```
#![cfg_attr(test, feature(test))]
```

## How To

to run
```
cargo run --release
```

to run tests
```
cargo +nightly test --release
```

to run benchmarks
```
cargo +nightly bench
```
