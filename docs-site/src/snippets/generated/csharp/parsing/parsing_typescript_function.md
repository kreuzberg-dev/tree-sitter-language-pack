---
id: fixture_csharp_parsing_typescript_function
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("function greet(name: string): string { return `hi ${name}`; }", new ProcessConfig { Language = "typescript" });

```
