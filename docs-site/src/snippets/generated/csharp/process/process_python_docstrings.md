```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("def greet(name):\n    \"\"\"Say hello to someone.\"\"\"\n    return f\"Hello {name}\"\n", new ProcessConfig { Docstrings = true, Language = "python" });

```
