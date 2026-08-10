```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("def greet(name):\n    return f'Hello, {name}!'\n", new ProcessConfig { Language = "python" });

```
