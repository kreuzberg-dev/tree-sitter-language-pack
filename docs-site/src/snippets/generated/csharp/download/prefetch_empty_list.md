---
id: fixture_csharp_prefetch_empty_list
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

TreeSitterLanguagePackConverter.Prefetch(new List<String>() {  });

```
