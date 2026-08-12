---
id: fixture_csharp_rust_chunking_process_detail
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("fn alpha() {}\n\nfn beta() {}\n\nfn gamma() {}\n\nfn delta() {}\n", new ProcessConfig { ChunkMaxSize = 30, Language = "rust" });

```
