```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("*.foreground: #ffffff\n", new ProcessConfig { Language = "xresources" });

```
