```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("module M {\n};\n", new ProcessConfig { Language = "idl" });

```
