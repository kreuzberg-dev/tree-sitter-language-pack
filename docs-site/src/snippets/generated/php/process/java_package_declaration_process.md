```php title="PHP"
<?php

use Tree\Sitter\Language\Pack\TreeSitterLanguagePack;
use Tree\Sitter\Language\Pack\Language;
use Tree\Sitter\Language\Pack\ProcessConfig;
use Tree\Sitter\Language\Pack\Tree;
$config = \Tree\Sitter\Language\Pack\ProcessConfig::from_json(json_encode(["language" => "java"]));
$result = TreeSitterLanguagePack::process("package com.example.widget;\n\npublic class Widget {\n    public String name() { return \"w\"; }\n}\n", $config);

```
