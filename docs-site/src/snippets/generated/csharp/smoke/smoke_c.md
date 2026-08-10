```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("int main() { return 0; }", new ProcessConfig { Language = "c" });

```
