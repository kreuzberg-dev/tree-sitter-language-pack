---
id: fixture_java_get_parser_python
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
        var parser = TreeSitterLanguagePack.getParser("python");
        System.out.println(parser);
    }
}

```
