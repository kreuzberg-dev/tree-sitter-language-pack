```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("function y = hello(x)\ny = x;\nend", new ProcessConfig { Language = "matlab" });

```
