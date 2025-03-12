fn main() {
    cc::Build::new()
        .file("src/config/config.c")
        .file("src/components/hardware.c")
        .compile("monitor");

    // linker do Cargo
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
    println!("cargo:rustc-link-lib=framework=DiskArbitration");
}