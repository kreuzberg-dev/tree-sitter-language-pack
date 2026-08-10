```java title="Java"
import io.xberg.treesitterlanguagepack.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var configJson = "{\"data_extraction\":true,\"language\":\"yaml\"}";
var config = JsonUtil.fromJson(configJson, ProcessConfig.class);
        var result = io.xberg.treesitterlanguagepack.TreeSitterLanguagePack.process("server:\n  host: localhost\n  port: 8080\n", config);
    }
}

```
