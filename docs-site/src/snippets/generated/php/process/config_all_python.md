```php title="PHP"
<?php

use Tree\Sitter\Language\Pack\TreeSitterLanguagePack;
use Tree\Sitter\Language\Pack\Language;
use Tree\Sitter\Language\Pack\ProcessConfig;
use Tree\Sitter\Language\Pack\Tree;
$config = \Tree\Sitter\Language\Pack\ProcessConfig::from_json(json_encode(["language" => "python"]));
$result = TreeSitterLanguagePack::process("# A comment\ndef greet(name):\n    \"\"\"Say hello.\"\"\"\n    return f'Hi {name}'\n\nimport os\n", $config);

```
