extern crate shaderc;

pub fn compile_shader_file(in_path: std::path::PathBuf, out_path: std::path::PathBuf) {
    let shader_kind = match in_path.extension() {
        Some(extension) => match extension.to_str().unwrap() {
            "vert" => shaderc::ShaderKind::Vertex,
            "frag" => shaderc::ShaderKind::Fragment,
            _ => panic!("Invalid extension!"),
        },
        _ => panic!("Invalid extension!"),
    };
    let source = std::fs::read_to_string(&in_path).unwrap();
    let mut compiler = shaderc::Compiler::new().unwrap();
    let compiled_file = compiler
        .compile_into_spirv(
            &source,
            shader_kind,
            in_path.to_str().unwrap(),
            "main",
            None,
        )
        .unwrap();
    std::fs::write(&out_path, compiled_file.as_binary_u8()).unwrap();
}

pub fn compile_shader_files(input_dir: std::path::PathBuf, output_dir: std::path::PathBuf) {
    std::fs::create_dir_all(output_dir.clone()).unwrap();
    for input_subdir in std::fs::read_dir(input_dir).unwrap() {
        let input_subdir = input_subdir.unwrap();
        if input_subdir.file_type().unwrap().is_file() {
            let input_path = input_subdir.path();
            let output_filename = input_path.file_name().unwrap().to_str().unwrap();
            let mut output_path = std::path::PathBuf::from(output_dir.clone());
            output_path.set_file_name(output_filename);
            output_path.set_extension("spv");
            compile_shader_file(input_path, output_path);
        }
    }
}
