```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("entity main is end main;", new ProcessConfig { Language = "vhdl" });

```
