extern crate cc;

// #[cfg(all(target_os = "windows", target_arch = "aarch64"))]
fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").map_or(false, |os| os == "windows")
        && std::env::var("CARGO_CFG_TARGET_ARCH").map_or(false, |arch| arch == "aarch64")
    {
        cc::Build::new()
            .include("minimp3/")
            .file("minimp3.c")
            .define("MINIMP3_IMPLEMENTATION", None)
            .define("MINIMP3_ONLY_MP3", None)
            .define("MINIMP3_NO_SIMD", None)
            .compile("minimp3");
    } else {
        cc::Build::new()
            .include("minimp3/")
            .file("minimp3.c")
            .define("MINIMP3_IMPLEMENTATION", None)
            .define("MINIMP3_ONLY_MP3", None)
            .compile("minimp3");
    }
}
