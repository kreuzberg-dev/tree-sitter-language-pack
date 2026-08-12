---
id: fixture_csharp_parse_empty_language
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using System;
using TreeSitterLanguagePack;

try
{
var result = TreeSitterLanguagePackConverter.Process("x = 1", new ProcessConfig { Language = "" });
}
catch (Exception error)
{
    Console.Error.WriteLine($"Call failed as expected: {error.Message}");
    return;
}
throw new InvalidOperationException("expected call to fail");

```
