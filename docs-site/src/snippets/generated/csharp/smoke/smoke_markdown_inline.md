```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("**bold** and *italic*", new ProcessConfig { Language = "markdown_inline" });

```
