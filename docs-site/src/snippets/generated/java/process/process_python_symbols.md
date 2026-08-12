---
id: fixture_java_process_python_symbols
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
        var configJson = "{\"language\":\"python\",\"symbols\":true}";
var config = JsonUtil.fromJson(configJson, ProcessConfig.class);
        var result = io.xberg.treesitterlanguagepack.TreeSitterLanguagePack.process("MY_CONST = 42\ndef helper(): pass\nclass Widget: pass\n", config);
        System.out.println(result);
    }
}

```
