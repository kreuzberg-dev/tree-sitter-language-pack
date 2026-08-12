---
id: fixture_java_prefetch_languages
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
        TreeSitterLanguagePack.prefetch(java.util.List.of("python"));
    }
}

```
