```java title="Java"
import io.xberg.treesitterlanguagepack.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var configJson = "{\"language\":\"python\"}";
var config = JsonUtil.fromJson(configJson, ProcessConfig.class);
        var result = io.xberg.treesitterlanguagepack.TreeSitterLanguagePack.process("# A comment\ndef greet(name):\n    \"\"\"Say hello.\"\"\"\n    return f'Hi {name}'\n\nimport os\n", config);
    }
}

```
