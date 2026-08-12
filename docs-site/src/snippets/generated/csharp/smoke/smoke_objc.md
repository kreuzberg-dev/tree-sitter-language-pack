---
id: fixture_csharp_smoke_objc
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("@interface Main @end", new ProcessConfig { Language = "objc" });

```
