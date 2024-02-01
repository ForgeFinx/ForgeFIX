const SOURCE_FILE: &str = "../forgefix-c/fix_client.c"; 
const OUT_NAME: &str = "fix_client"; 

const INCLUDES: &[&str] = &[
    "../forgefix-c",
];

fn main() {
    println!("cargo:rerun-if-changed={}", SOURCE_FILE);
    cc::Build::new()
        .file(SOURCE_FILE)
        .includes(INCLUDES)
        .compile(OUT_NAME);

    println!("cargo:rustc-link-search-native=../target/debug");
    println!("cargo:rustc-link-lib=dylib=forgefix_c");
}


