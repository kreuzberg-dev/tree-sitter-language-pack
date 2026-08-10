```php title="PHP"
<?php

use Tree\Sitter\Language\Pack\TreeSitterLanguagePack;
use Tree\Sitter\Language\Pack\Language;
use Tree\Sitter\Language\Pack\ProcessConfig;
use Tree\Sitter\Language\Pack\Tree;
$config = \Tree\Sitter\Language\Pack\ProcessConfig::from_json(json_encode(["language" => "javascript"]));
$result = TreeSitterLanguagePack::process("export function greet(name) {\n  return `Hello \${name}`;\n}\n\nexport const VERSION = '1.0';\n", $config);

```
