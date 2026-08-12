---
id: fixture_csharp_smoke_hlsl
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("float4 main() : SV_Target { return 0; }", new ProcessConfig { Language = "hlsl" });

```
