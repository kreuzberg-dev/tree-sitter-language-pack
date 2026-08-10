```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("/dts-v1/;\n/ { };", new ProcessConfig { Language = "devicetree" });

```
