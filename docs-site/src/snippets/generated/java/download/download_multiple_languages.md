---
id: fixture_java_download_multiple_languages
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
        var result = TreeSitterLanguagePack.download(java.util.List.of("python", "rust"));
        System.out.println(result);
    }
}

```
