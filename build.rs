fn main() {
    cc::Build::new()
        .warnings(true)
        .flag("-Wall")
        .flag("-Wextra")
        .file("src/print.c")
        .include("src")
        .compile("libprint.a");
}
