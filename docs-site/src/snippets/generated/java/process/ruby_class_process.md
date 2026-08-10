```java title="Java"
import io.xberg.treesitterlanguagepack.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var configJson = "{\"language\":\"ruby\"}";
var config = JsonUtil.fromJson(configJson, ProcessConfig.class);
        var result = io.xberg.treesitterlanguagepack.TreeSitterLanguagePack.process("require 'json'\n\nclass Greeter\n  def greet(name)\n    \"Hello #{name}\"\n  end\nend\n", config);
    }
}

```
