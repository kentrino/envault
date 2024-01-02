pub fn out_path(input: &str) -> String {
    let dirname = std::path::Path::new(input)
        .parent()
        .unwrap()
        .to_str()
        .unwrap();
    let basename = std::path::Path::new(input)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();
    let ext = std::path::Path::new(input)
        .extension()
        .unwrap()
        .to_str()
        .unwrap();
    if dirname.is_empty() {
        return format!("{}.enc.{}", basename, ext);
    }
    format!("{}/{}.enc.{}", dirname, basename, ext)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_out_path() {
        assert_eq!(super::out_path("/tmp/test.json"), "/tmp/test.enc.json");
        assert_eq!(super::out_path("./test.json"), "./test.enc.json");
        assert_eq!(super::out_path("test.json"), "test.enc.json");
    }
}
