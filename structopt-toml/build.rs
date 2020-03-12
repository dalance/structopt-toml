fn main() {
    if cfg!(test) {
        skeptic::generate_doc_tests(&["../README.md"]);
    }
}
