```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("const App = () => <div />;", new ProcessConfig { Language = "tsx" });

```
