```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("DESCRIPTION = \"hello\"", new ProcessConfig { Language = "bitbake" });

```
