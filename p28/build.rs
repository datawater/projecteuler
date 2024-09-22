use cc::*;

fn main() {
    Build::new()
        .file("src/main.c")
        .opt_level(4)
        .flag("-Wall")
        .flag("-Wextra")
        .flag("-Werror")
        .flag("-mtune=native")
        .flag("-march=native")
        .compile("main")
    ;

    println!("cargo::rerun-if-changed=src/hello.c");
}