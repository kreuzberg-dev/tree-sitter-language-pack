```php title="PHP"
<?php

use Tree\Sitter\Language\Pack\TreeSitterLanguagePack;
use Tree\Sitter\Language\Pack\Language;
use Tree\Sitter\Language\Pack\ProcessConfig;
use Tree\Sitter\Language\Pack\Tree;
$config = \Tree\Sitter\Language\Pack\ProcessConfig::from_json(json_encode(["chunkMaxSize" => 30, "language" => "python"]));
$result = TreeSitterLanguagePack::process("def alpha():\n    pass\n\ndef beta():\n    pass\n\ndef gamma():\n    pass\n\ndef delta():\n    pass\n", $config);

```
