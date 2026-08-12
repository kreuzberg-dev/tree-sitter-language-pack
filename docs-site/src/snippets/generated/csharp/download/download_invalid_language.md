---
id: fixture_csharp_download_invalid_language
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using System;
using System.Text.Json;
using TreeSitterLanguagePack;

var ConfigOptions = new JsonSerializerOptions { PropertyNameCaseInsensitive = true };
try
{
var result = TreeSitterLanguagePackConverter.Download(new List<String>() { JsonSerializer.Deserialize<String>("\"zzz_definitely_not_a_real_language_xyz\"", ConfigOptions)! });
}
catch (Exception error)
{
    Console.Error.WriteLine($"Call failed as expected: {error.Message}");
    return;
}
throw new InvalidOperationException("expected call to fail");

```
