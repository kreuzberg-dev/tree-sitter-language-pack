```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("G0 X0\n", new ProcessConfig { Language = "gcode" });

```
