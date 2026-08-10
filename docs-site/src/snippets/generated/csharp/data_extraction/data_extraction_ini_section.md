```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("[database]\nhost=localhost\nport=5432\n", new ProcessConfig { DataExtraction = true, Language = "ini" });

```
