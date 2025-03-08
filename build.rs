use std::path::PathBuf;

// based on find_cargo_target_dir from sdl3-sys
fn top_level_cargo_target_dir() -> std::path::PathBuf {
    use std::path::PathBuf;
    let pkg_name = std::env::var("CARGO_PKG_NAME").unwrap();
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let mut target = PathBuf::from(&out_dir);
    let pop = |target: &mut PathBuf| assert!(target.pop(), "malformed OUT_DIR: {:?}", out_dir);
    while !target
        .file_name()
        .unwrap()
        .to_string_lossy()
        .contains(&pkg_name)
    {
        pop(&mut target);
    }
    pop(&mut target);
    pop(&mut target);
    target
}

fn main() {
    let ghostty_location = std::env::var("GHOSTTY_LOCATION").expect("Missing GHOSTTY_LOCATION containing libghostty.so (or equivalent)");

    println!("cargo:rustc-link-search={}", ghostty_location);
    println!("cargo:rustc-link-lib=ghostty");

    let bindings = bindgen::Builder::default()
        .header("src/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings!");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    #[cfg(not(windows))]
    let name = "libghostty.so";
    #[cfg(windows)]
    let name = "ghostty.dll";

    std::fs::copy(
        format!("{}/{}", ghostty_location, name),
        top_level_cargo_target_dir().join(name),
    )
    .expect("Copy of ghostty failed!");
}
