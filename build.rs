extern crate rae_shader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let in_path = std::path::PathBuf::from("src/shaders/glsl/");
    let out_path = std::path::PathBuf::from("src/shaders/spirv/");
    println!(
        "cargo:rerun-if-changed={}**",
        in_path.to_str().unwrap_or("")
    );
    rae_shader::compile_shaders_into_spirv(in_path, out_path)?;
    Ok(())
}
