---
id: fixture_java_registry_get_parser_alias
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
        var parser = TreeSitterLanguagePack.getParser("shell");
        System.out.println(parser);
    }
}

```
