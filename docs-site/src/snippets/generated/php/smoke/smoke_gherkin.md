```php title="PHP"
<?php

use Tree\Sitter\Language\Pack\TreeSitterLanguagePack;
use Tree\Sitter\Language\Pack\Language;
use Tree\Sitter\Language\Pack\ProcessConfig;
use Tree\Sitter\Language\Pack\Tree;
$config = \Tree\Sitter\Language\Pack\ProcessConfig::from_json(json_encode(["language" => "gherkin"]));
$result = TreeSitterLanguagePack::process("Feature: Calculator\n  Scenario: Add numbers\n    Given I have entered 1\n    When I add 2\n    Then the result should be 3\n", $config);

```
