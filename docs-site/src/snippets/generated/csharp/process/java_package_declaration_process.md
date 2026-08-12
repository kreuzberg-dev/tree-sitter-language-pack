---
id: fixture_csharp_java_package_declaration_process
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("package com.example.widget;\n\npublic class Widget {\n    public String name() { return \"w\"; }\n}\n", new ProcessConfig { Language = "java" });

```
