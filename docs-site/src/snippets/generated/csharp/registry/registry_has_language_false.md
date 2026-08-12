---
id: fixture_csharp_registry_has_language_false
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.HasLanguage("nonexistent");

```
