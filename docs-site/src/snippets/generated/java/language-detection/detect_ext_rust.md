---
id: fixture_java_detect_ext_rust
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
        var result = TreeSitterLanguagePack.detectLanguageFromExtension("rs");
        System.out.println(result);
    }
}

```
