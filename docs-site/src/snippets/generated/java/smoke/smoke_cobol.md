---
id: fixture_java_smoke_cobol
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
        var configJson = "{\"language\":\"cobol\"}";
var config = JsonUtil.fromJson(configJson, ProcessConfig.class);
        var result = io.xberg.treesitterlanguagepack.TreeSitterLanguagePack.process("       IDENTIFICATION DIVISION.\n       PROGRAM-ID. HELLO.", config);
        System.out.println(result);
    }
}

```
