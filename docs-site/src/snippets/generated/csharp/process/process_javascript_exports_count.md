---
id: fixture_csharp_process_javascript_exports_count
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("export function greet() { return 'hi'; }\nexport const VERSION = '1.0';\nexport default class App {}\n", new ProcessConfig { Language = "javascript" });

```
