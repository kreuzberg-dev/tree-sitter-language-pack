```java title="Java"
import io.xberg.treesitterlanguagepack.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var configJson = "{\"language\":\"gherkin\"}";
var config = JsonUtil.fromJson(configJson, ProcessConfig.class);
        var result = io.xberg.treesitterlanguagepack.TreeSitterLanguagePack.process("Feature: Calculator\n  Scenario: Add numbers\n    Given I have entered 1\n    When I add 2\n    Then the result should be 3\n", config);
    }
}

```
