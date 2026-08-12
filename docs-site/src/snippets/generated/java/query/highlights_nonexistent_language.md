---
id: fixture_java_highlights_nonexistent_language
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
        var result = TreeSitterLanguagePack.getHighlightsQuery("zzz_nonexistent_lang");
        System.out.println(result);
    }
}

```
