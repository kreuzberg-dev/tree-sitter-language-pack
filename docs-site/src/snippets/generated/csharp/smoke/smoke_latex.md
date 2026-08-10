```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("\\documentclass{article}\n\\begin{document}\nHello\n\\end{document}", new ProcessConfig { Language = "latex" });

```
