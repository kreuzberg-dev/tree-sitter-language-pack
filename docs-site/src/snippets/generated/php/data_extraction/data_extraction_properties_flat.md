```php title="PHP"
<?php

use Tree\Sitter\Language\Pack\TreeSitterLanguagePack;
use Tree\Sitter\Language\Pack\Language;
use Tree\Sitter\Language\Pack\ProcessConfig;
use Tree\Sitter\Language\Pack\Tree;
$config = \Tree\Sitter\Language\Pack\ProcessConfig::from_json(json_encode(["dataExtraction" => true, "language" => "properties"]));
$result = TreeSitterLanguagePack::process("host=localhost\nport=8080\n", $config);

```
