```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("<!ELEMENT server (host, port)>\n<!ELEMENT host (#PCDATA)>\n", new ProcessConfig { DataExtraction = true, Language = "dtd" });

```
