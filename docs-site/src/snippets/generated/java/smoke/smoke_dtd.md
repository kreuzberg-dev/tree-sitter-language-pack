```java title="Java"
import io.xberg.treesitterlanguagepack.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var configJson = "{\"language\":\"dtd\"}";
var config = JsonUtil.fromJson(configJson, ProcessConfig.class);
        var result = io.xberg.treesitterlanguagepack.TreeSitterLanguagePack.process("<!ELEMENT note (body)>", config);
    }
}

```
