```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("#include <stdio.h>\n\nint main() {\n    printf(\"hello\");\n    return 0;\n}\n", new ProcessConfig { Language = "c" });

```
