```java title="Java"
import io.xberg.treesitterlanguagepack.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var configJson = "{\"language\":\"m68k\"}";
var config = JsonUtil.fromJson(configJson, ProcessConfig.class);
        var result = io.xberg.treesitterlanguagepack.TreeSitterLanguagePack.process(" move.l d0,d1\n", config);
    }
}

```
