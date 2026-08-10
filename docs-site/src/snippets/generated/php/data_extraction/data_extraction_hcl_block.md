```php title="PHP"
<?php

use Tree\Sitter\Language\Pack\TreeSitterLanguagePack;
use Tree\Sitter\Language\Pack\Language;
use Tree\Sitter\Language\Pack\ProcessConfig;
use Tree\Sitter\Language\Pack\Tree;
$config = \Tree\Sitter\Language\Pack\ProcessConfig::from_json(json_encode(["dataExtraction" => true, "language" => "hcl"]));
$result = TreeSitterLanguagePack::process("resource \"aws_instance\" \"web\" {\n  ami = \"ami-123\"\n  instance_type = \"t2.micro\"\n}\n", $config);

```
