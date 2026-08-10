```php title="PHP"
<?php

use Tree\Sitter\Language\Pack\TreeSitterLanguagePack;
use Tree\Sitter\Language\Pack\Language;
use Tree\Sitter\Language\Pack\ProcessConfig;
use Tree\Sitter\Language\Pack\Tree;
$config = \Tree\Sitter\Language\Pack\ProcessConfig::from_json(json_encode(["dataExtraction" => true, "language" => "kdl"]));
$result = TreeSitterLanguagePack::process("server {\n  host \"localhost\"\n  port 8080\n}\n", $config);

```
