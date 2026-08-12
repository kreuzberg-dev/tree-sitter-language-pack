---
id: fixture_csharp_kotlin_package_class_process
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("package foo.bar\n\nclass Widget {\n    fun greet(): String = \"hi\"\n}\n", new ProcessConfig { Language = "kotlin" });

```
