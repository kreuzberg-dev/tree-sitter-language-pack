---
id: fixture_csharp_smoke_haxe
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("class Main { static function main() {} }", new ProcessConfig { Language = "haxe" });

```
