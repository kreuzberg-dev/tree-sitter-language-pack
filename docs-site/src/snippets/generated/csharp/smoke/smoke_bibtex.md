```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("@article{key, title={A}}", new ProcessConfig { Language = "bibtex" });

```
