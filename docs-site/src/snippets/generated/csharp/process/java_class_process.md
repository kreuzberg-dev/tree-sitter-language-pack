---
id: fixture_csharp_java_class_process
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("import java.util.List;\n\npublic class Greeter {\n    public String greet(String name) {\n        return \"Hello \" + name;\n    }\n}\n", new ProcessConfig { Language = "java" });

```
