---
id: fixture_csharp_get_parser_unknown
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
var parser = TreeSitterLanguagePackConverter.GetParser("nonexistent_xyz");
}
catch (Exception error)
{
    Console.Error.WriteLine($"Call failed as expected: {error.Message}");
    return;
}
throw new InvalidOperationException("expected call to fail");

```
