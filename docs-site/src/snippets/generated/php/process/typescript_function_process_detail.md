---
id: fixture_php_typescript_function_process_detail
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

declare(strict_types=1);

require_once __DIR__ . '/vendor/autoload.php';

use Tree\Sitter\Language\Pack\TreeSitterLanguagePack;
use Tree\Sitter\Language\Pack\Language;
use Tree\Sitter\Language\Pack\ProcessConfig;
use Tree\Sitter\Language\Pack\Tree;
$config = \Tree\Sitter\Language\Pack\ProcessConfig::from_json(json_encode(["language" => "typescript"]));
$result = TreeSitterLanguagePack::process("import { readFile } from 'fs';\n\nfunction greet(name: string): string {\n    return `Hello, \${name}!`;\n}\n", $config);

```
