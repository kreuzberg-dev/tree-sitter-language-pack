```java title="Java"
import io.xberg.treesitterlanguagepack.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var configJson = "{}";
var config = JsonUtil.fromJson(configJson, PackConfig.class);
        TreeSitterLanguagePack.init(config);
    }
}

```
