fn main() {
    println!("cargo:rerun-if-env-changed=SHOWCONSOLE");
    println!("cargo::rustc-check-cfg=cfg(showconsole)");
    if let Ok(v) = std::env::var("SHOWCONSOLE") {
        if v == "1" {
            println!("cargo:rustc-cfg=showconsole");
        }
    }
}