---
id: fixture_csharp_smoke_thrift
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("service HelloService {}", new ProcessConfig { Language = "thrift" });

```
