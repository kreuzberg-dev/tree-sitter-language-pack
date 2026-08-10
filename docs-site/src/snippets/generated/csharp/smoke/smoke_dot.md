```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("digraph G { A -> B; }", new ProcessConfig { Language = "dot" });

```
