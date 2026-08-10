```php title="PHP"
<?php

use Tree\Sitter\Language\Pack\TreeSitterLanguagePack;
use Tree\Sitter\Language\Pack\Language;
use Tree\Sitter\Language\Pack\ProcessConfig;
use Tree\Sitter\Language\Pack\Tree;
$config = \Tree\Sitter\Language\Pack\ProcessConfig::from_json(json_encode(["language" => "typescript"]));
$result = TreeSitterLanguagePack::process("import { readFile } from 'fs';\n\nfunction greet(name: string): string {\n    return `Hello, \${name}!`;\n}\n", $config);

```
