```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("export void main() {}", new ProcessConfig { Language = "ispc" });

```
