---
id: fixture_java_process_unknown_language
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
        try {
        var configJson = "{\"language\":\"nonexistent_xyz\"}";
var config = JsonUtil.fromJson(configJson, ProcessConfig.class);
        var result = io.xberg.treesitterlanguagepack.TreeSitterLanguagePack.process("x = 1", config);
        System.out.println(result);
        } catch (Exception error) {
            System.err.println("Call failed as expected: " + error.getMessage());
            return;
        }
        throw new AssertionError("expected call to fail");
    }
}

```
