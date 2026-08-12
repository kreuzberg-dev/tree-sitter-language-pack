---
id: fixture_java_registry_get_language_alias
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
        var language = TreeSitterLanguagePack.getLanguage("shell");
        System.out.println(language);
    }
}

```
