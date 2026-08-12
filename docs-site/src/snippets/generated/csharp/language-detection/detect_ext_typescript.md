---
id: fixture_csharp_detect_ext_typescript
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.DetectLanguageFromExtension("ts");

```
