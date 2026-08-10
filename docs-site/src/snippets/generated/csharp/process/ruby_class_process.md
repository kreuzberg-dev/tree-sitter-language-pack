```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("require 'json'\n\nclass Greeter\n  def greet(name)\n    \"Hello #{name}\"\n  end\nend\n", new ProcessConfig { Language = "ruby" });

```
