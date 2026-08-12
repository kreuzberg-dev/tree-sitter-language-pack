---
id: fixture_java_get_language_python
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
        var language = TreeSitterLanguagePack.getLanguage("python");
        System.out.println(language);
    }
}

```
