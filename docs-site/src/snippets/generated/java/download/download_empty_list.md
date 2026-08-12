---
id: fixture_java_download_empty_list
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
        var result = TreeSitterLanguagePack.download(java.util.List.of());
        System.out.println(result);
    }
}

```
