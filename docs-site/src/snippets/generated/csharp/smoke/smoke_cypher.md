```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("MATCH (n) RETURN n\n", new ProcessConfig { Language = "cypher" });

```
