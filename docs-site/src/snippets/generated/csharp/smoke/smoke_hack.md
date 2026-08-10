```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("<?hh\nfunction main(): void {}", new ProcessConfig { Language = "hack" });

```
