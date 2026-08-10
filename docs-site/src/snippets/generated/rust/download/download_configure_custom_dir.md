```rust title="Rust"
use tree_sitter_language_pack::configure;

fn main() {
    let config_json: serde_json::Value = serde_json::from_str(r#"{"cache_dir":"/tmp/tslp_test_cache"}"#).unwrap();
    let config = serde_json::from_value(config_json).unwrap();
    let _ = configure(&config);
}

```
