```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("object \"C\" {\n  code {\n  }\n}\n", new ProcessConfig { Language = "yul" });

```
