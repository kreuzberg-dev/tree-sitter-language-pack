---
id: fixture_csharp_smoke_devicetree
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("/dts-v1/;\n/ { };", new ProcessConfig { Language = "devicetree" });

```
