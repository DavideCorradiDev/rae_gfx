extern crate shaderc;

pub fn compile_shaders_in_directory(input_dir: std::path::PathBuf, output_dir: std::path::PathBuf) {
    std::fs::create_dir_all(output_dir).unwrap();
}
