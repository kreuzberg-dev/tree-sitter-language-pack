---
id: fixture_csharp_smoke_ruby
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("puts 'hello'", new ProcessConfig { Language = "ruby" });

```
