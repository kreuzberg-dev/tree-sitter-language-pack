---
id: fixture_java_parsing_javascript_variable
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
        var result = io.xberg.treesitterlanguagepack.TreeSitterLanguagePack.process("const x = 1;", config);
        System.out.println(result);
    }
}

```
