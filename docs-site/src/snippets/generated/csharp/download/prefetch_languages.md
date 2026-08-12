---
id: fixture_csharp_prefetch_languages
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using System.Text.Json;
using TreeSitterLanguagePack;

var ConfigOptions = new JsonSerializerOptions { PropertyNameCaseInsensitive = true };
TreeSitterLanguagePackConverter.Prefetch(new List<String>() { JsonSerializer.Deserialize<String>("\"python\"", ConfigOptions)! });

```
