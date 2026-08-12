---
id: fixture_java_error_handling_empty_source
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
        var configJson = "{\"language\":\"javascript\"}";
var config = JsonUtil.fromJson(configJson, ProcessConfig.class);
        var result = io.xberg.treesitterlanguagepack.TreeSitterLanguagePack.process("", config);
        System.out.println(result);
    }
}

```
