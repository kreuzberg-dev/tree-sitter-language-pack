```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("---\n---\n<p>hello</p>", new ProcessConfig { Language = "astro" });

```
