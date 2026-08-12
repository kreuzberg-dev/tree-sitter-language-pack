---
id: fixture_csharp_smoke_capnp
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("@0xabcdef1234567890;", new ProcessConfig { Language = "capnp" });

```
