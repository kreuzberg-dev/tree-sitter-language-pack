```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("<script>let x = 1;</script>", new ProcessConfig { Language = "svelte" });

```
