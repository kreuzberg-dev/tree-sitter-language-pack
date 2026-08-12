---
id: fixture_csharp_c_function_process
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("#include <stdio.h>\n\nint main() {\n    printf(\"hello\");\n    return 0;\n}\n", new ProcessConfig { Language = "c" });

```
