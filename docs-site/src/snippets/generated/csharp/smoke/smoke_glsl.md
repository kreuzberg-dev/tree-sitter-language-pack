---
id: fixture_csharp_smoke_glsl
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("void main() { gl_Position = vec4(0.0); }", new ProcessConfig { Language = "glsl" });

```
