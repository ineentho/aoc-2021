#[cfg(test)]
pub fn read_test_resource(filename: &str) -> String {
    use std::{fs, path::PathBuf};

    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources/test/");
    d.push(filename);

    fs::read_to_string(d).unwrap().trim().to_string()
}
