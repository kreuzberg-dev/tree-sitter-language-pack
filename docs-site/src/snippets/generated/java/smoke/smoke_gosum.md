---
id: fixture_java_smoke_gosum
language: java
target: java
level: typecheck
requires: []
side_effect: safe
---

```java title="Java"
import io.xberg.treesitterlanguagepack.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var configJson = "{\"language\":\"gosum\"}";
var config = JsonUtil.fromJson(configJson, ProcessConfig.class);
        var result = io.xberg.treesitterlanguagepack.TreeSitterLanguagePack.process("example.com/pkg v1.0.0 h1:abc=", config);
        System.out.println(result);
    }
}

```
