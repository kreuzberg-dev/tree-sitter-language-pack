```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("import fs from 'fs';\nimport path from 'path';\n\nfunction process(input) {\n    return input.trim();\n}\n", new ProcessConfig { Language = "javascript" });

```
