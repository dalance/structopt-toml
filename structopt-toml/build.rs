use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("../README.md");
    if path.exists() {
        skeptic::generate_doc_tests(&[path]);
    }
}
