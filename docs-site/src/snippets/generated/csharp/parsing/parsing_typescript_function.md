```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("function greet(name: string): string { return `hi ${name}`; }", new ProcessConfig { Language = "typescript" });

```
