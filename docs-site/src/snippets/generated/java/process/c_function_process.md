---
id: fixture_java_c_function_process
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
        var configJson = "{\"language\":\"c\"}";
var config = JsonUtil.fromJson(configJson, ProcessConfig.class);
        var result = io.xberg.treesitterlanguagepack.TreeSitterLanguagePack.process("#include <stdio.h>\n\nint main() {\n    printf(\"hello\");\n    return 0;\n}\n", config);
        System.out.println(result);
    }
}

```
