---
id: fixture_csharp_smoke_smali
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process(".class public LMain;\n.super Ljava/lang/Object;", new ProcessConfig { Language = "smali" });

```
