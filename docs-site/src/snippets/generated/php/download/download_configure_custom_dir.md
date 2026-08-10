```php title="PHP"
<?php

use Tree\Sitter\Language\Pack\TreeSitterLanguagePack;
use Tree\Sitter\Language\Pack\Language;
use Tree\Sitter\Language\Pack\PackConfig;
use Tree\Sitter\Language\Pack\Tree;
$config = \Tree\Sitter\Language\Pack\PackConfig::from_json(json_encode(["cacheDir" => "/tmp/tslp_test_cache"]));
TreeSitterLanguagePack::configure($config);

```
