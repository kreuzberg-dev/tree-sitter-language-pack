---
id: fixture_csharp_detect_path_dotfile
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.DetectLanguageFromPath(".gitignore");

```
