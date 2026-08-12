---
id: fixture_csharp_error_handling_get_language_empty_string
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
var language = TreeSitterLanguagePackConverter.GetLanguage("");
}
catch (Exception error)
{
    Console.Error.WriteLine($"Call failed as expected: {error.Message}");
    return;
}
throw new InvalidOperationException("expected call to fail");

```
