---
id: fixture_java_error_detect_content_empty
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
        var result = TreeSitterLanguagePack.detectLanguageFromContent("");
        System.out.println(result);
    }
}

```
