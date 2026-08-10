```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("[*.rs]\nindent_style = space\nindent_size = 4\n", new ProcessConfig { DataExtraction = true, Language = "editorconfig" });

```
