```php title="PHP"
<?php

use Tree\Sitter\Language\Pack\TreeSitterLanguagePack;
use Tree\Sitter\Language\Pack\Language;
use Tree\Sitter\Language\Pack\ProcessConfig;
use Tree\Sitter\Language\Pack\Tree;
$config = \Tree\Sitter\Language\Pack\ProcessConfig::from_json(json_encode(["chunkMaxSize" => 30, "language" => "rust"]));
$result = TreeSitterLanguagePack::process("fn alpha() {}\n\nfn beta() {}\n\nfn gamma() {}\n\nfn delta() {}\n", $config);

```
