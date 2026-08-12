---
id: fixture_csharp_smoke_zig
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("pub fn main() void {}", new ProcessConfig { Language = "zig" });

```
