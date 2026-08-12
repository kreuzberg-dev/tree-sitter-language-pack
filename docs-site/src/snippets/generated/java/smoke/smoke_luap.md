---
id: fixture_java_smoke_luap
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
        var configJson = "{\"language\":\"luap\"}";
var config = JsonUtil.fromJson(configJson, ProcessConfig.class);
        var result = io.xberg.treesitterlanguagepack.TreeSitterLanguagePack.process("[a-z]+", config);
        System.out.println(result);
    }
}

```
