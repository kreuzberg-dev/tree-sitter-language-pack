---
id: fixture_csharp_smoke_cuda
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("__global__ void kernel() {}", new ProcessConfig { Language = "cuda" });

```
