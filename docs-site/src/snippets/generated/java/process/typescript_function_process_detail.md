```java title="Java"
import io.xberg.treesitterlanguagepack.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var configJson = "{\"language\":\"typescript\"}";
var config = JsonUtil.fromJson(configJson, ProcessConfig.class);
        var result = io.xberg.treesitterlanguagepack.TreeSitterLanguagePack.process("import { readFile } from 'fs';\n\nfunction greet(name: string): string {\n    return `Hello, ${name}!`;\n}\n", config);
    }
}

```
