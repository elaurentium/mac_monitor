fn main() {
    cc::Build::new()
        .file("src/config/config.c")
        .compile("config");
}