```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("SELECT ?s WHERE { ?s ?p ?o }", new ProcessConfig { Language = "sparql" });

```
