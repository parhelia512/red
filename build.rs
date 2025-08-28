fn main() {
    println!("cargo:rerun-if-env-changed=SHOWCONSOLE");
    if let Ok(v) = std::env::var("SHOWCONSOLE") {
        if v == "1" {
            println!("cargo:rustc-cfg=showconsole");
        }
    }
}