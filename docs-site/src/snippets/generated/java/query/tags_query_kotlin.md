---
id: fixture_java_tags_query_kotlin
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
        var result = TreeSitterLanguagePack.getTagsQuery("kotlin");
        System.out.println(result);
    }
}

```
