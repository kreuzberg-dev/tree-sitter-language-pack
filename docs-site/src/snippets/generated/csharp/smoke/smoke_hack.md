---
id: fixture_csharp_smoke_hack
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("<?hh\nfunction main(): void {}", new ProcessConfig { Language = "hack" });

```
