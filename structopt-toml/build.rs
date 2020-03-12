#[cfg(test)]
fn main() {
    skeptic::generate_doc_tests(&["../README.md"]);
}

#[cfg(not(test))]
fn main() {}
