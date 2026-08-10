```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("type Query { hello: String }", new ProcessConfig { Language = "graphql" });

```
