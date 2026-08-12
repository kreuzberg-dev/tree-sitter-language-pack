---
id: fixture_csharp_smoke_verilog
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("module main; endmodule", new ProcessConfig { Language = "verilog" });

```
