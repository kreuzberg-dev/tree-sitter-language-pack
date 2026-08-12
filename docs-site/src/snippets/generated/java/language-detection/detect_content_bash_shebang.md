---
id: fixture_java_detect_content_bash_shebang
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
        var result = TreeSitterLanguagePack.detectLanguageFromContent("#!/bin/bash\necho hi");
        System.out.println(result);
    }
}

```
