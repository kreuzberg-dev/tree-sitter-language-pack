---
id: fixture_csharp_detect_path_go_nested
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.DetectLanguageFromPath("lib/server.go");

```
