extern crate rae_shader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let in_dir = std::path::PathBuf::from("src/shaders/glsl");
    let out_dir = std::path::PathBuf::from("src/shaders/gen/spirv");
    println!(
        "cargo:rerun-if-changed={}/**",
        in_dir.to_str().unwrap_or("")
    );
    rae_shader::compile_shaders_into_spirv(in_dir, out_dir)?;
    Ok(())
}
