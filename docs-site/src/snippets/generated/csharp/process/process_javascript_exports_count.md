```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("export function greet() { return 'hi'; }\nexport const VERSION = '1.0';\nexport default class App {}\n", new ProcessConfig { Language = "javascript" });

```
