```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("pub struct MyConfig {\n    pub name: String,\n    pub value: i32,\n}\n\nimpl MyConfig {\n    pub fn new() -> Self {\n        Self { name: String::new(), value: 0 }\n    }\n}\n", new ProcessConfig { Language = "rust" });

```
