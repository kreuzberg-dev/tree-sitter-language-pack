---
id: fixture_csharp_smoke_ballerina
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("public function main() {\n}\n", new ProcessConfig { Language = "ballerina" });

```
