```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("= Title\n\nParagraph.", new ProcessConfig { Language = "asciidoc" });

```
