```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("module main; endmodule", new ProcessConfig { Language = "verilog" });

```
