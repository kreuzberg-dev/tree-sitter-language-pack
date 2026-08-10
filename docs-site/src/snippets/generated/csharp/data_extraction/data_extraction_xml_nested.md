```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("<config><host>localhost</host><port>8080</port></config>", new ProcessConfig { DataExtraction = true, Language = "xml" });

```
