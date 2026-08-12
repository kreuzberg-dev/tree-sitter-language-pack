---
id: fixture_csharp_smoke_c
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("int main() { return 0; }", new ProcessConfig { Language = "c" });

```
