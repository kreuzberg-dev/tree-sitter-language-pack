---
id: fixture_java_indents_query_cmake
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
        var result = TreeSitterLanguagePack.getIndentsQuery("cmake");
        System.out.println(result);
    }
}

```
