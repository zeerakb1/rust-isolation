fn main() {
    cc::Build::new()
        .file("library/library.c")
        .compile("libxzs.a");
}