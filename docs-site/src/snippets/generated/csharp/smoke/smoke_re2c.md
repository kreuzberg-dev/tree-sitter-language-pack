```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("/*!re2c\n  [a-z]+ { return; }\n*/", new ProcessConfig { Language = "re2c" });

```
