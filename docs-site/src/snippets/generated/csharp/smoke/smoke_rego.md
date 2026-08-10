```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("package main\ndefault allow = false", new ProcessConfig { Language = "rego" });

```
