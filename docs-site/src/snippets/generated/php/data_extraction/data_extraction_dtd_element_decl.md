```php title="PHP"
<?php

use Tree\Sitter\Language\Pack\TreeSitterLanguagePack;
use Tree\Sitter\Language\Pack\Language;
use Tree\Sitter\Language\Pack\ProcessConfig;
use Tree\Sitter\Language\Pack\Tree;
$config = \Tree\Sitter\Language\Pack\ProcessConfig::from_json(json_encode(["dataExtraction" => true, "language" => "dtd"]));
$result = TreeSitterLanguagePack::process("<!ELEMENT server (host, port)>\n<!ELEMENT host (#PCDATA)>\n", $config);

```
