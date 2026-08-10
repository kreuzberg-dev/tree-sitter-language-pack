```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("package foo.bar\n\nclass Widget {\n    fun greet(): String = \"hi\"\n}\n", new ProcessConfig { Language = "kotlin" });

```
