include!("src/bindings/build.rs"); // for build_bindings()

fn main() -> std::io::Result<()> {
    build_bindings()?;
    Ok(())
}
