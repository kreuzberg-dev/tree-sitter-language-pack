```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("package main\nfunc main() {}", new ProcessConfig { Language = "go" });

```
