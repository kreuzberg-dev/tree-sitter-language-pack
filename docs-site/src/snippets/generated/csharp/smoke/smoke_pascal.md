```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("program Hello; begin end.", new ProcessConfig { Language = "pascal" });

```
