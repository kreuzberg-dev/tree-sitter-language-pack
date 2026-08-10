```java title="Java"
import io.xberg.treesitterlanguagepack.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var configJson = "{\"language\":\"haskell\"}";
var config = JsonUtil.fromJson(configJson, ProcessConfig.class);
        var result = io.xberg.treesitterlanguagepack.TreeSitterLanguagePack.process("{-aaaaaaaaaaaaaa aaaa}\n    {-aaa (aaaaaaaaaa [aaaaaaaaaaaaa aaa", config);
    }
}

```
