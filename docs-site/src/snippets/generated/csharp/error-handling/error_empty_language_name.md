---
id: fixture_csharp_error_empty_language_name
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
var result = TreeSitterLanguagePackConverter.Process("hello", new ProcessConfig { Language = "" });
}
catch (Exception error)
{
    Console.Error.WriteLine($"Call failed as expected: {error.Message}");
    return;
}
throw new InvalidOperationException("expected call to fail");

```
