---
id: fixture_csharp_process_python_symbols
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("MY_CONST = 42\ndef helper(): pass\nclass Widget: pass\n", new ProcessConfig { Language = "python", Symbols = true });

```
