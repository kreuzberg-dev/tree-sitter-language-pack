```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("<!ELEMENT note (body)>", new ProcessConfig { Language = "dtd" });

```
