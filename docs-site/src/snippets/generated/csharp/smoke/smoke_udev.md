```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("ACTION==\"add\", KERNEL==\"sd*\"", new ProcessConfig { Language = "udev" });

```
