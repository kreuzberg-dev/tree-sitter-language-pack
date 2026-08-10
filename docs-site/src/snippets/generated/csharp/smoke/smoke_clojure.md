```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("(def x 1)", new ProcessConfig { Language = "clojure" });

```
