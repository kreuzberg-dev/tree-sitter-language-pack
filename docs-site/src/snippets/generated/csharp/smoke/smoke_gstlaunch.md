```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("fakesrc ! fakesink", new ProcessConfig { Language = "gstlaunch" });

```
