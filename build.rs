#[rustversion::nightly]
fn main() {
    println!("cargo:rustc-cfg=test");
}

#[rustversion::not(nightly)]
fn main() { }