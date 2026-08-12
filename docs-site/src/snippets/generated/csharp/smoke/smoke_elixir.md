---
id: fixture_csharp_smoke_elixir
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("IO.puts(\"hello\")", new ProcessConfig { Language = "elixir" });

```
