```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("# module docstring\nimport os\n\ndef hello():\n    # greeting\n    print('hello')\n\ndef world():\n    print('world')\n", new ProcessConfig { Language = "python" });

```
