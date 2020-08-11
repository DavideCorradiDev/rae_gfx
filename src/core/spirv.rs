extern crate shaderc;

pub fn compile_shaders_in_directory(
    input_dir: std::path::PathBuf,
    mut output_dir: std::path::PathBuf,
) {
    let mut compiler = shaderc::Compiler::new().unwrap();

    std::fs::create_dir_all(output_dir.clone()).unwrap();
    for input_subdir in std::fs::read_dir(input_dir).unwrap() {
        let input_subdir = input_subdir.unwrap();
        if input_subdir.file_type().unwrap().is_file() {
            let input_path = input_subdir.path();
            let shader_kind = input_path.extension().and_then(|extension| {
                match extension.to_string_lossy().as_ref() {
                    "vert" => Some(shaderc::ShaderKind::Vertex),
                    "frag" => Some(shaderc::ShaderKind::Fragment),
                    _ => None,
                }
            });
            if let Some(shader_kind) = shader_kind {
                let source = std::fs::read_to_string(&input_path).unwrap();
                let compiled_file = compiler
                    .compile_into_spirv(
                        &source,
                        shader_kind,
                        input_path.to_str().unwrap(),
                        "main",
                        None,
                    )
                    .unwrap();
                let output_filename = input_path.file_name().unwrap().to_str().unwrap();
                output_dir.set_file_name(output_filename);
                output_dir.set_extension("spv");
                std::fs::write(&output_dir, compiled_file.as_binary_u8()).unwrap();
            }
        }
    }
}
