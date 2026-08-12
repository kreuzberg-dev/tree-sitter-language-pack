---
id: fixture_java_registry_has_language_alias
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
        var result = TreeSitterLanguagePack.hasLanguage("shell");
        System.out.println(result);
    }
}

```
