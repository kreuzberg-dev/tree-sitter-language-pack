---
priority: high
---

# Binding-Level Configuration Only

Language bindings (spikard-py, spikard-node, spikard-rb, spikard-php) must NOT duplicate
middleware logic. All middleware lives in Rust (tower-http). Bindings only expose configuration
APIs that construct ServerConfig and pass it to the Rust server. Python uses PyO3, TypeScript
uses napi-rs, Ruby uses magnus, PHP uses ext-php-rs.
