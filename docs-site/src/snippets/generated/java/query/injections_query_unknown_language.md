---
id: fixture_java_injections_query_unknown_language
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
        var result = TreeSitterLanguagePack.getInjectionsQuery("nonexistent_xyz");
        System.out.println(result);
    }
}

```
