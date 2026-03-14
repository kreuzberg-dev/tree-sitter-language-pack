fn main() {
    let langs = ts_pack_core::available_languages();
    assert!(!langs.is_empty(), "Expected languages to be available");
    println!("Available languages: {}", langs.len());

    assert!(ts_pack_core::has_language("rust"), "rust should be available");

    let tree = ts_pack_core::parse_string("rust", b"fn main() {}").expect("parse should succeed");
    assert!(!ts_pack_core::tree_has_error_nodes(&tree), "tree should have no errors");

    println!("Rust smoke test passed");
}
