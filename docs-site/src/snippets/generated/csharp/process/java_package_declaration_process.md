```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("package com.example.widget;\n\npublic class Widget {\n    public String name() { return \"w\"; }\n}\n", new ProcessConfig { Language = "java" });

```
