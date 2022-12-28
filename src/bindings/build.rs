use bindgen::builder;
use std::env;
use std::io::Result;
use std::path::Path;

fn build_bindings() -> Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let cargo_root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let bindings = builder()
        .header(
            Path::new(&cargo_root_dir)
                .join("src/bindings/wrapper.h")
                .to_string_lossy(),
        )
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_function("_getch")
        .allowlist_function("_getwch")
        .generate()
        .expect("无法生成binding");

    bindings
        .write_to_file(Path::new(&out_dir).join("bindings.rs"))
        .expect("无法写入binding.rs");

    println!("cargo:rerun-if-changed=c_src/wrapper.h");
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
