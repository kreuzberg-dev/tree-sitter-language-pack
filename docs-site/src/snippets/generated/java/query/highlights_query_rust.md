---
id: fixture_java_highlights_query_rust
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
        var result = TreeSitterLanguagePack.getHighlightsQuery("rust");
        System.out.println(result);
    }
}

```
