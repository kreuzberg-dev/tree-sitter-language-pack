```php title="PHP"
<?php

use Tree\Sitter\Language\Pack\TreeSitterLanguagePack;
use Tree\Sitter\Language\Pack\Language;
use Tree\Sitter\Language\Pack\PackConfig;
use Tree\Sitter\Language\Pack\Tree;
$config = \Tree\Sitter\Language\Pack\PackConfig::from_json('{}');
TreeSitterLanguagePack::init($config);

```
