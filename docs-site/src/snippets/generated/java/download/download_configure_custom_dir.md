```java title="Java"
import io.xberg.treesitterlanguagepack.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var configJson = "{\"cache_dir\":\"/tmp/tslp_test_cache\"}";
var config = JsonUtil.fromJson(configJson, PackConfig.class);
        TreeSitterLanguagePack.configure(config);
    }
}

```
