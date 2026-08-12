---
id: fixture_csharp_smoke_cmake
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("cmake_minimum_required(VERSION 3.0)", new ProcessConfig { Language = "cmake" });

```
