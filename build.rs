fn main() {
    prost_build::Config::new()
        // 生成Bytes 而非缺省的 Vec
        .bytes(&["."])
        .type_attribute(".", "#[derive(PartialOrd)]")
        .out_dir("src/pb")
        .compile_protos(&["abi.proto"], &["."])
        .unwrap();
}
