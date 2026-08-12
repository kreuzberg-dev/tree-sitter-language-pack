---
id: fixture_java_process_python_comments
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
        var configJson = "{\"comments\":true,\"language\":\"python\"}";
var config = JsonUtil.fromJson(configJson, ProcessConfig.class);
        var result = io.xberg.treesitterlanguagepack.TreeSitterLanguagePack.process("# This is a comment\n# Another comment\ndef hello():\n    # inline comment\n    pass\n", config);
        System.out.println(result);
    }
}

```
