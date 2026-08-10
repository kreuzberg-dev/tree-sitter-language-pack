```php title="PHP"
<?php

use Tree\Sitter\Language\Pack\TreeSitterLanguagePack;
use Tree\Sitter\Language\Pack\Language;
use Tree\Sitter\Language\Pack\ProcessConfig;
use Tree\Sitter\Language\Pack\Tree;
$config = \Tree\Sitter\Language\Pack\ProcessConfig::from_json(json_encode(["comments" => true, "language" => "python"]));
$result = TreeSitterLanguagePack::process("# This is a comment\n# Another comment\ndef hello():\n    # inline comment\n    pass\n", $config);

```
