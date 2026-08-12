---
id: fixture_csharp_parsing_go_function
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("package main\nfunc main() {}", new ProcessConfig { Language = "go" });

```
