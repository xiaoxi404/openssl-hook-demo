fn main() {
    cc::Build::new()
        .file("src/hook.c")
        .include("ssl")
        .define("_GNU_SOURCE", None)
        .compile("hook");

    println!("cargo::rerun-if-changed=src/hook.c");
    println!("cargo::rerun-if-changed=build.rs");
}
