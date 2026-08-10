```java title="Java"
import io.xberg.treesitterlanguagepack.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var configJson = "{\"language\":\"java\"}";
var config = JsonUtil.fromJson(configJson, ProcessConfig.class);
        var result = io.xberg.treesitterlanguagepack.TreeSitterLanguagePack.process("import java.util.List;\n\npublic class Greeter {\n    public String greet(String name) {\n        return \"Hello \" + name;\n    }\n}\n", config);
    }
}

```
