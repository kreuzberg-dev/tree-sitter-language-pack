```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("resource \"aws_instance\" \"web\" {\n  ami = \"ami-123\"\n  instance_type = \"t2.micro\"\n}\n", new ProcessConfig { DataExtraction = true, Language = "hcl" });

```
