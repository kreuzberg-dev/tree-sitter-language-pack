---
id: fixture_csharp_download_multiple_languages
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
var result = TreeSitterLanguagePackConverter.Download(new List<String>() { JsonSerializer.Deserialize<String>("\"python\"", ConfigOptions)!, JsonSerializer.Deserialize<String>("\"rust\"", ConfigOptions)! });

```
