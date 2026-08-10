```java title="Java"
import io.xberg.treesitterlanguagepack.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var configJson = "{\"language\":\"python\"}";
var config = JsonUtil.fromJson(configJson, ProcessConfig.class);
        var result = io.xberg.treesitterlanguagepack.TreeSitterLanguagePack.process("class Calculator:\n    def add(self, a, b):\n        return a + b\n\n    def subtract(self, a, b):\n        return a - b\n", config);
    }
}

```
