```php title="PHP"
<?php

use Tree\Sitter\Language\Pack\TreeSitterLanguagePack;
use Tree\Sitter\Language\Pack\Language;
use Tree\Sitter\Language\Pack\ProcessConfig;
use Tree\Sitter\Language\Pack\Tree;
$config = \Tree\Sitter\Language\Pack\ProcessConfig::from_json(json_encode(["language" => "c"]));
$result = TreeSitterLanguagePack::process("#include <stdio.h>\n\nint main() {\n    printf(\"hello\");\n    return 0;\n}\n", $config);

```
