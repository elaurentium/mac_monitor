fn main() {
    cc::Build::new()
        .file("src/config/config.c")
        .file("src/components/hardware.c")
        .flag("-arch")
        .flag("arm64")
        .compile("monitor");

    // linker do Cargo
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
    println!("cargo:rustc-link-lib=framework=DiskArbitration");
}