```php title="PHP"
<?php

use Tree\Sitter\Language\Pack\TreeSitterLanguagePack;
use Tree\Sitter\Language\Pack\Language;
use Tree\Sitter\Language\Pack\ProcessConfig;
use Tree\Sitter\Language\Pack\Tree;
$config = \Tree\Sitter\Language\Pack\ProcessConfig::from_json(json_encode(["language" => "rust"]));
$result = TreeSitterLanguagePack::process("fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n", $config);

```
