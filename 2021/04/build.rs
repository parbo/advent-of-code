fn main() {
    // only run if target os is windows
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() != "windows" {
        return;
    }

    println!("cargo:rustc-link-arg=Advapi32.lib");
}
